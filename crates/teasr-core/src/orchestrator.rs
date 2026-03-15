use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::{info, warn};

use crate::capture;
use crate::server::ManagedServer;
use crate::types::{CaptureResult, OutputFormat, ResolvedConfig, SceneConfig};

/// Run all scenes in order and return capture results.
pub async fn run(config: &ResolvedConfig) -> Result<Vec<CaptureResult>> {
    let output_dir = PathBuf::from(&config.output.dir);
    std::fs::create_dir_all(&output_dir)
        .with_context(|| format!("failed to create output dir: {}", output_dir.display()))?;

    // Start server if configured
    let _server = match &config.server {
        Some(server_config) => Some(ManagedServer::start(server_config).await?),
        None => None,
    };

    let mut results = Vec::new();

    for (i, scene) in config.scenes.iter().enumerate() {
        let scene_name = scene.name().to_string();
        info!("[{}/{}] capturing: {} ({})", i + 1, config.scenes.len(), scene_name, scene.scene_type());

        let formats = scene
            .formats()
            .as_ref()
            .unwrap_or(&config.output.formats);

        let mut files = Vec::new();

        for format in formats {
            match format {
                OutputFormat::Png => {
                    let path = output_dir.join(format!("{scene_name}.png"));
                    capture_scene(scene, &path, config).await?;
                    files.push(path.display().to_string());
                }
                OutputFormat::Gif => {
                    warn!("GIF capture requires frame sequence - using single-frame GIF");
                    let png_path = output_dir.join(format!("{scene_name}.png"));
                    capture_scene(scene, &png_path, config).await?;
                    let gif_path = output_dir.join(format!("{scene_name}.gif"));
                    crate::convert::gif::png_to_gif(&png_path, &gif_path)?;
                    files.push(gif_path.display().to_string());
                }
                OutputFormat::Mp4 => {
                    warn!("MP4 output requires ffmpeg in PATH - skipping for now");
                }
            }
        }

        results.push(CaptureResult {
            scene_name,
            files,
        });
    }

    info!("all scenes captured successfully");
    Ok(results)
}

async fn capture_scene(
    scene: &SceneConfig,
    output_path: &Path,
    config: &ResolvedConfig,
) -> Result<()> {
    match scene {
        SceneConfig::Web {
            url,
            viewport,
            actions,
            ..
        } => {
            let vp = viewport.as_ref().unwrap_or(&config.viewport);
            let full_url = if let Some(server) = &config.server {
                if url.starts_with('/') {
                    format!("{}{}", server.url.trim_end_matches('/'), url)
                } else {
                    url.clone()
                }
            } else {
                url.clone()
            };

            capture::web::capture(
                &full_url,
                vp,
                actions.as_deref().unwrap_or(&[]),
                output_path,
            )
            .await
        }
        SceneConfig::Screen {
            display,
            region,
            setup,
            delay,
            ..
        } => {
            if let Some(setup_cmd) = setup {
                info!("running setup: {setup_cmd}");
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg(setup_cmd)
                    .status()
                    .context("setup command failed")?;
            }
            if let Some(ms) = delay {
                tokio::time::sleep(std::time::Duration::from_millis(*ms)).await;
            }
            capture::screen::capture(*display, region.as_ref(), output_path)
        }
        SceneConfig::Terminal {
            command,
            theme,
            cols,
            name,
            ..
        } => {
            let theme = theme.as_deref().unwrap_or("dracula");
            let cols = cols.unwrap_or(80);
            capture::terminal::capture(
                command,
                cols,
                theme,
                name.as_deref(),
                output_path,
            )
        }
    }
}
