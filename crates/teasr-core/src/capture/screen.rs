use anyhow::{Context, Result};
use std::path::Path;

use crate::types::Region;

/// Capture a screen region to PNG.
pub fn capture(
    display: Option<u32>,
    region: Option<&Region>,
    output_path: &Path,
) -> Result<()> {
    let screens = xcap::Monitor::all().context("failed to enumerate monitors")?;

    let monitor = if let Some(idx) = display {
        screens
            .into_iter()
            .nth(idx as usize)
            .context("display index out of range")?
    } else {
        screens
            .into_iter()
            .next()
            .context("no monitors found")?
    };

    let image = monitor.capture_image().context("failed to capture screen")?;

    let image = if let Some(r) = region {
        let sub = image::DynamicImage::ImageRgba8(image)
            .crop_imm(r.x, r.y, r.width, r.height);
        match sub {
            image::DynamicImage::ImageRgba8(img) => img,
            other => other.to_rgba8(),
        }
    } else {
        image
    };

    image
        .save(output_path)
        .with_context(|| format!("failed to save screenshot to {}", output_path.display()))?;

    Ok(())
}
