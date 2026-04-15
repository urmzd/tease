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
        let img_frame = imgref::ImgVec::new(pixels, max_width as usize, max_height as usize);
        collector
            .add_frame_rgba(i, img_frame, timestamp)
            .with_context(|| format!("failed to add frame {i}"))?;
        timestamp += frame.duration_ms as f64 / 1000.0;
    }
    drop(collector);

    write_handle
        .join()
        .map_err(|_| anyhow::anyhow!("GIF writer thread panicked"))??;

    Ok(())
}
