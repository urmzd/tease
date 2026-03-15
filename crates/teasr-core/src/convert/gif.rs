use anyhow::{Context, Result};
use std::path::Path;

/// Convert a single PNG to a single-frame GIF using gifski.
pub fn png_to_gif(png_path: &Path, gif_path: &Path) -> Result<()> {
    let png_data = std::fs::read(png_path)
        .with_context(|| format!("failed to read {}", png_path.display()))?;

    let img = image::load_from_memory(&png_data).context("failed to decode PNG")?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

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

    // Add single frame
    let pixels: Vec<rgb::RGBA8> = rgba
        .pixels()
        .map(|p| rgb::RGBA8::new(p[0], p[1], p[2], p[3]))
        .collect();
    let frame = imgref::ImgVec::new(pixels, width as usize, height as usize);
    collector
        .add_frame_rgba(0, frame, 0.0)
        .context("failed to add frame")?;
    drop(collector);

    write_handle
        .join()
        .map_err(|_| anyhow::anyhow!("GIF writer thread panicked"))??;

    Ok(())
}
