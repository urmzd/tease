//! Streamsafe pipeline stages for teasr capture output.
//!
//! Encodes a captured frame stream into multiple output formats concurrently
//! using streamsafe's `.broadcast()` / `.broadcast3()` fan-out primitives.
//!
//! Today each format is a whole-stream sink (GIF accumulates all frames; PNG
//! keeps the last). The entry point is [`write_outputs`], a drop-in replacement
//! for teasr-core's sequential `write_outputs` that runs each format writer on
//! its own task.

use std::path::{Path, PathBuf};

use streamsafe::{PipelineBuilder, Result as SsResult, Sink, Source, StreamSafeError};

use crate::types::{CapturedFrame, GifConfig, OutputFormat};

mod sinks;

pub use sinks::FormatSink;

/// Yields `CapturedFrame`s from an in-memory buffer.
pub struct FrameVecSource {
    iter: std::vec::IntoIter<CapturedFrame>,
}

impl FrameVecSource {
    pub fn new(frames: Vec<CapturedFrame>) -> Self {
        Self {
            iter: frames.into_iter(),
        }
    }
}

impl Source for FrameVecSource {
    type Output = CapturedFrame;

    async fn produce(&mut self) -> SsResult<Option<CapturedFrame>> {
        Ok(self.iter.next())
    }
}

/// Concurrent multi-format output writer.
///
/// Mirrors the semantics of teasr-core's `write_outputs` but runs each format's
/// encoding on its own task: slow GIF encoding does not block PNG writing and
/// vice versa. Returns the set of files written.
pub async fn write_outputs(
    frames: Vec<CapturedFrame>,
    scene_name: &str,
    formats: &[OutputFormat],
    output_dir: &Path,
) -> anyhow::Result<Vec<String>> {
    if frames.is_empty() {
        return Ok(Vec::new());
    }

    let mut jobs = Vec::new();
    for format in formats {
        if let Some(job) = FormatSink::for_format(format, scene_name, output_dir) {
            jobs.push(job);
        }
    }

    match jobs.len() {
        0 => Ok(Vec::new()),
        1 => run_single(frames, jobs).await,
        2 => run_broadcast2(frames, jobs).await,
        3 => run_broadcast3(frames, jobs).await,
        _ => run_many(frames, jobs).await,
    }
}

async fn run_single(
    frames: Vec<CapturedFrame>,
    mut jobs: Vec<FormatSink>,
) -> anyhow::Result<Vec<String>> {
    let sink = jobs.pop().unwrap();
    let path = sink.output_path().to_string_lossy().to_string();
    PipelineBuilder::from(FrameVecSource::new(frames))
        .into(sink)
        .run()
        .await
        .map_err(|e| anyhow::anyhow!("pipeline failed: {e}"))?;
    Ok(vec![path])
}

async fn run_broadcast2(
    frames: Vec<CapturedFrame>,
    mut jobs: Vec<FormatSink>,
) -> anyhow::Result<Vec<String>> {
    let b = jobs.pop().unwrap();
    let a = jobs.pop().unwrap();
    let paths = vec![
        a.output_path().to_string_lossy().to_string(),
        b.output_path().to_string_lossy().to_string(),
    ];
    PipelineBuilder::from(FrameVecSource::new(frames))
        .broadcast(a, b)
        .run()
        .await
        .map_err(|e| anyhow::anyhow!("pipeline failed: {e}"))?;
    Ok(paths)
}

async fn run_broadcast3(
    frames: Vec<CapturedFrame>,
    mut jobs: Vec<FormatSink>,
) -> anyhow::Result<Vec<String>> {
    let c = jobs.pop().unwrap();
    let b = jobs.pop().unwrap();
    let a = jobs.pop().unwrap();
    let paths = vec![
        a.output_path().to_string_lossy().to_string(),
        b.output_path().to_string_lossy().to_string(),
        c.output_path().to_string_lossy().to_string(),
    ];
    PipelineBuilder::from(FrameVecSource::new(frames))
        .broadcast3(a, b, c)
        .run()
        .await
        .map_err(|e| anyhow::anyhow!("pipeline failed: {e}"))?;
    Ok(paths)
}

async fn run_many(
    frames: Vec<CapturedFrame>,
    jobs: Vec<FormatSink>,
) -> anyhow::Result<Vec<String>> {
    // 4+ formats: fall back to sequential processing (vanishingly rare in practice).
    // Streamsafe's typed broadcast doesn't generalize past 3 without variadic macros.
    let paths: Vec<String> = jobs
        .iter()
        .map(|j| j.output_path().to_string_lossy().to_string())
        .collect();
    for mut sink in jobs {
        for frame in frames.iter().cloned() {
            sink.consume(frame)
                .await
                .map_err(|e| anyhow::anyhow!("sink consume failed: {e}"))?;
        }
        sink.finish()
            .await
            .map_err(|e| anyhow::anyhow!("sink finish failed: {e}"))?;
    }
    Ok(paths)
}

/// Helper: wrap any `std::error::Error` as a [`StreamSafeError`].
pub(crate) fn other_err<E: std::error::Error + Send + Sync + 'static>(e: E) -> StreamSafeError {
    StreamSafeError::other(e)
}

/// Internal: build a GIF sink. Public via [`FormatSink::for_format`].
pub(crate) fn gif_sink(path: PathBuf, config: GifConfig) -> FormatSink {
    FormatSink::Gif {
        path,
        config,
        frames: Vec::new(),
    }
}

/// Internal: build a PNG sink. Public via [`FormatSink::for_format`].
pub(crate) fn png_sink(path: PathBuf) -> FormatSink {
    FormatSink::Png { path, last: None }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::PngConfig;
    use tempfile::TempDir;

    fn frame(png: &[u8], duration_ms: u64) -> CapturedFrame {
        CapturedFrame {
            png_data: png.to_vec(),
            duration_ms,
        }
    }

    #[tokio::test]
    async fn writes_single_png() {
        // Use a real 1x1 PNG so image decoding in the GIF path would succeed
        // if we routed to GIF — for this test we only exercise PNG.
        let png = minimal_png();
        let frames = vec![frame(&png, 100), frame(&png, 100)];
        let dir = TempDir::new().unwrap();

        let files = write_outputs(
            frames,
            "scene",
            &[OutputFormat::Png(PngConfig::default())],
            dir.path(),
        )
        .await
        .unwrap();

        assert_eq!(files.len(), 1);
        let written = std::fs::read(dir.path().join("scene.png")).unwrap();
        assert_eq!(written, png);
    }

    #[tokio::test]
    async fn no_formats_returns_empty() {
        let files = write_outputs(vec![frame(b"x", 10)], "scene", &[], std::path::Path::new("/tmp"))
            .await
            .unwrap();
        assert!(files.is_empty());
    }

    #[tokio::test]
    async fn no_frames_returns_empty() {
        let files = write_outputs(
            vec![],
            "scene",
            &[OutputFormat::Png(PngConfig::default())],
            std::path::Path::new("/tmp"),
        )
        .await
        .unwrap();
        assert!(files.is_empty());
    }

    /// Smallest valid 1x1 transparent PNG.
    fn minimal_png() -> Vec<u8> {
        vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00,
            0x00, 0x1F, 0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x44, 0x41, 0x54, 0x78,
            0x9C, 0x62, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00,
            0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
        ]
    }
}
