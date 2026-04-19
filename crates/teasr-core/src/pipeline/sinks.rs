use std::path::{Path, PathBuf};

use streamsafe::{Result as SsResult, Sink};

use super::{gif_sink, other_err, png_sink};
use crate::types::{CapturedFrame, GifConfig, OutputFormat};

/// Discriminated sink covering the output formats teasr writes today.
///
/// Each variant stages frames during `consume` and does the expensive work in
/// `finish`: GIF accumulates then encodes; PNG keeps only the final frame then
/// writes it. MP4 is a no-op — upstream is expected to skip it with a warning.
pub enum FormatSink {
    Gif {
        path: PathBuf,
        config: GifConfig,
        frames: Vec<CapturedFrame>,
    },
    Png {
        path: PathBuf,
        last: Option<CapturedFrame>,
    },
}

impl FormatSink {
    /// Build a sink for a configured output format. Returns `None` for formats
    /// teasr cannot emit via the pipeline (currently MP4).
    pub fn for_format(format: &OutputFormat, scene_name: &str, output_dir: &Path) -> Option<Self> {
        match format {
            OutputFormat::Gif(cfg) => Some(gif_sink(
                output_dir.join(format!("{scene_name}.gif")),
                cfg.clone(),
            )),
            OutputFormat::Png(_) => Some(png_sink(output_dir.join(format!("{scene_name}.png")))),
            OutputFormat::Mp4(_) => {
                tracing::warn!("MP4 output requires ffmpeg in PATH — skipping");
                None
            }
        }
    }

    /// Where this sink will write its output.
    pub fn output_path(&self) -> &Path {
        match self {
            FormatSink::Gif { path, .. } => path,
            FormatSink::Png { path, .. } => path,
        }
    }
}

impl Sink for FormatSink {
    type Input = CapturedFrame;

    async fn consume(&mut self, frame: CapturedFrame) -> SsResult<()> {
        match self {
            FormatSink::Gif { frames, .. } => frames.push(frame),
            FormatSink::Png { last, .. } => *last = Some(frame),
        }
        Ok(())
    }

    async fn finish(&mut self) -> SsResult<()> {
        match self {
            FormatSink::Gif {
                path,
                config,
                frames,
            } if !frames.is_empty() => {
                let frames_taken = std::mem::take(frames);
                let path = path.clone();
                let config = config.clone();
                // Gifski encoding is CPU-bound and blocking; hop to a blocking thread.
                tokio::task::spawn_blocking(move || {
                    crate::convert::gif::frames_to_gif(&frames_taken, &path, &config)
                })
                .await
                .map_err(other_err)?
                .map_err(|e| other_err(anyhow_to_io(e)))
            }
            FormatSink::Gif { .. } => Ok(()),
            FormatSink::Png { path, last } => {
                if let Some(frame) = last.take() {
                    let path = path.clone();
                    tokio::task::spawn_blocking(move || std::fs::write(&path, &frame.png_data))
                        .await
                        .map_err(other_err)?
                        .map_err(other_err)?;
                }
                Ok(())
            }
        }
    }
}

/// Flatten an `anyhow::Error` into something that satisfies
/// `std::error::Error + Send + Sync`, preserving the message chain.
fn anyhow_to_io(err: anyhow::Error) -> std::io::Error {
    std::io::Error::other(format!("{err:#}"))
}
