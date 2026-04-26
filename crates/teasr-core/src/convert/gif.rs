use anyhow::{Context, Result};
use std::path::Path;

use crate::types::{CapturedFrame, GifConfig};

/// Assemble multiple captured frames into an animated GIF.
pub fn frames_to_gif(frames: &[CapturedFrame], gif_path: &Path, config: &GifConfig) -> Result<()> {
    if frames.is_empty() {
        anyhow::bail!("no frames to encode");
    }

    // Merge consecutive frames with identical PNG data to reduce encoding work
    let merged: Vec<CapturedFrame> = {
        let mut out: Vec<CapturedFrame> = Vec::new();
        for frame in frames {
            if let Some(last) = out.last_mut() {
                if last.png_data == frame.png_data {
                    last.duration_ms += frame.duration_ms;
                    continue;
                }
            }
            out.push(frame.clone());
        }
        out
    };

    // Decode all frames and find max dimensions so every frame fits
    let decoded: Vec<image::RgbaImage> = merged
        .iter()
        .enumerate()
        .map(|(i, f)| {
            image::load_from_memory(&f.png_data)
                .with_context(|| format!("failed to decode frame {i}"))
                .map(|img| img.to_rgba8())
        })
        .collect::<Result<_>>()?;

    let mut max_width = 0u32;
    let mut max_height = 0u32;
    for img in &decoded {
        let (w, h) = img.dimensions();
        max_width = max_width.max(w);
        max_height = max_height.max(h);
    }

    let repeat = match config.repeat {
        None | Some(0) => gifski::Repeat::Infinite,
        Some(n) => gifski::Repeat::Finite(n),
    };

    let (collector, writer) = gifski::new(gifski::Settings {
        width: Some(max_width),
        height: Some(max_height),
        quality: config.quality,
        fast: config.fast,
        repeat,
    })?;

    let gif_path_owned = gif_path.to_path_buf();
    let write_handle = std::thread::spawn(move || -> Result<()> {
        let file = std::fs::File::create(&gif_path_owned)
            .with_context(|| format!("failed to create {}", gif_path_owned.display()))?;
        writer
            .write(file, &mut gifski::progress::NoProgress {})
            .context("GIF write failed")?;
        Ok(())
    });

    let mut timestamp = 0.0_f64;
    let mut last_canvas_pixels: Option<Vec<rgb::RGBA8>> = None;
    for (i, (rgba, frame)) in decoded.iter().zip(merged.iter()).enumerate() {
        // Pad smaller frames onto a max-size canvas (top-left aligned)
        let canvas = if rgba.dimensions() != (max_width, max_height) {
            let mut canvas = image::RgbaImage::new(max_width, max_height);
            image::imageops::overlay(&mut canvas, rgba, 0, 0);
            canvas
        } else {
            rgba.clone()
        };
        let pixels: Vec<rgb::RGBA8> = canvas
            .pixels()
            .map(|p| rgb::RGBA8::new(p[0], p[1], p[2], p[3]))
            .collect();
        let img_frame = imgref::ImgVec::new(
            pixels.clone(),
            max_width as usize,
            max_height as usize,
        );
        collector
            .add_frame_rgba(i, img_frame, timestamp)
            .with_context(|| format!("failed to add frame {i}"))?;
        timestamp += frame.duration_ms as f64 / 1000.0;
        last_canvas_pixels = Some(pixels);
    }
    // gifski uses the most recent pts gap as the trailing frame's delay, so
    // a single sentinel inherits the last real frame's full duration and
    // visually doubles the end-hold. Append two sentinels — both visually
    // identical to the last real frame — at `timestamp` and `timestamp + 10ms`.
    // The first closes out the real last frame's intended duration; the second
    // pulls gifski's trailing-delay heuristic down to 10 ms before the loop.
    if let Some(pixels) = last_canvas_pixels {
        let img_a = imgref::ImgVec::new(
            pixels.clone(),
            max_width as usize,
            max_height as usize,
        );
        collector
            .add_frame_rgba(merged.len(), img_a, timestamp)
            .context("failed to add trailing sentinel frame")?;
        let img_b = imgref::ImgVec::new(pixels, max_width as usize, max_height as usize);
        collector
            .add_frame_rgba(merged.len() + 1, img_b, timestamp + 0.01)
            .context("failed to add trailing terminator frame")?;
    }
    drop(collector);

    write_handle
        .join()
        .map_err(|_| anyhow::anyhow!("GIF writer thread panicked"))??;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CapturedFrame;
    use image::{ImageBuffer, ImageFormat, Rgba};
    use std::io::Cursor;
    use tempfile::TempDir;

    fn solid_png(r: u8, g: u8, b: u8) -> Vec<u8> {
        let img = ImageBuffer::from_pixel(2, 2, Rgba([r, g, b, 255]));
        let mut bytes = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .unwrap();
        bytes
    }

    /// gifski derives each frame's display delay from the gap to the next
    /// frame's pts. Without trailing sentinels, the final frame's intended
    /// duration is silently truncated. This test asserts the last frame is
    /// held for at least its requested duration.
    #[test]
    fn last_frame_duration_is_honored() {
        let frames = vec![
            CapturedFrame {
                png_data: solid_png(255, 0, 0),
                duration_ms: 100,
            },
            CapturedFrame {
                png_data: solid_png(0, 255, 0),
                duration_ms: 2000,
            },
        ];
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("hold.gif");
        frames_to_gif(&frames, &path, &GifConfig::default()).unwrap();

        let bytes = std::fs::read(&path).unwrap();
        let delays = gif_frame_delays_cs(&bytes);
        // gifski dedupes the trailing identical sentinels, so the encoded
        // GIF has just the two distinct visual states. The last frame's
        // delay should be approximately the intended 2000 ms (200 cs),
        // give or take a few cs for the ~10 ms sentinel offset and gifski's
        // centisecond rounding.
        let last = *delays.last().expect("GIF has no frames");
        assert!(
            (198..=205).contains(&last),
            "expected last frame ~200 cs, got {last} (all delays: {delays:?})"
        );
    }

    /// Decode per-frame delays (in centiseconds) from a GIF byte stream.
    /// Walks the format directly to avoid pulling in another decoder dep.
    fn gif_frame_delays_cs(bytes: &[u8]) -> Vec<u16> {
        let mut delays = Vec::new();
        let mut i = 0;
        while i + 8 < bytes.len() {
            // Graphic Control Extension: 0x21, 0xF9, block size (4), packed,
            // delay-lo, delay-hi, transparent-color, block terminator (0).
            if bytes[i] == 0x21 && bytes[i + 1] == 0xF9 && bytes[i + 2] == 0x04 {
                let lo = bytes[i + 4] as u16;
                let hi = bytes[i + 5] as u16;
                delays.push((hi << 8) | lo);
                i += 8;
            } else {
                i += 1;
            }
        }
        delays
    }
}
