use anyhow::{Context, Result};
use std::path::Path;

use crate::types::CapturedFrame;

/// Assemble multiple captured frames into an animated GIF.
pub fn frames_to_gif(frames: &[CapturedFrame], gif_path: &Path) -> Result<()> {
    if frames.is_empty() {
        anyhow::bail!("no frames to encode");
    }

    // Decode first frame to get dimensions
    let first_img = image::load_from_memory(&frames[0].png_data).context("failed to decode first frame")?;
    let first_rgba = first_img.to_rgba8();
    let (width, height) = first_rgba.dimensions();

    let (collector, writer) = gifski::new(gifski::Settings {
        width: Some(width),
        height: Some(height),
        quality: 90,
        fast: false,
        repeat: gifski::Repeat::Infinite,
    })?;

    let gif_path_owned = gif_path.to_path_buf();
    let write_handle = std::thread::spawn(move || -> Result<()> {
        let file = std::fs::File::create(&gif_path_owned)
            .with_context(|| format!("failed to create {}", gif_path_owned.display()))?;
        writer.write(file, &mut gifski::progress::NoProgress {}).context("GIF write failed")?;
        Ok(())
    });

    let mut timestamp = 0.0_f64;
    for (i, frame) in frames.iter().enumerate() {
        let img = image::load_from_memory(&frame.png_data).context("failed to decode frame PNG")?;
        let rgba = img.to_rgba8();
        // Resize if dimensions don't match the first frame
        let rgba = if rgba.dimensions() != (width, height) {
            image::imageops::resize(&rgba, width, height, image::imageops::FilterType::Lanczos3)
        } else {
            rgba
        };
        let pixels: Vec<rgb::RGBA8> = rgba
            .pixels()
            .map(|p| rgb::RGBA8::new(p[0], p[1], p[2], p[3]))
            .collect();
        let img_frame = imgref::ImgVec::new(pixels, width as usize, height as usize);
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
