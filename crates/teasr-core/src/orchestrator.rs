use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::{info, warn};

use crate::capture;
use crate::server::ManagedServer;
use crate::types::{CaptureResult, CapturedFrame, OutputFormat, ResolvedConfig, SceneConfig};

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

        match scene {
            SceneConfig::Terminal {
                steps,
                theme,
                cols,
                rows,
                name,
                frame_duration,
                ..
            } => {
                let has_gif = formats.iter().any(|f| matches!(f, OutputFormat::Gif));
                let has_png = formats.iter().any(|f| matches!(f, OutputFormat::Png));

                if !has_gif && !has_png {
                    warn!("terminal scene '{}' has no GIF or PNG format", scene_name);
                }

                let theme_name = theme.as_deref().unwrap_or("dracula");
                let cols = cols.unwrap_or(80);
                let rows = rows.unwrap_or(24);
                let fd = frame_duration.unwrap_or(100);

                info!("recording session: {} steps, {}x{}", steps.len(), cols, rows);

                let captured_frames = capture::terminal::capture_session(
                    cols,
                    rows,
                    theme_name,
                    name.as_deref(),
                    steps,
                    fd,
                )?;

                info!("captured {} frames", captured_frames.len());

                if has_gif && !captured_frames.is_empty() {
                    let gif_path = output_dir.join(format!("{scene_name}.gif"));
                    crate::convert::gif::frames_to_gif(&captured_frames, &gif_path)?;
                    files.push(gif_path.display().to_string());
                }

                if has_png && !captured_frames.is_empty() {
                    let png_path = output_dir.join(format!("{scene_name}.png"));
                    capture::terminal::write_last_frame_png(&captured_frames, &png_path)?;
                    files.push(png_path.display().to_string());
                }
            }
            SceneConfig::Web {
                url,
                viewport,
                actions,
                frame_duration,
                ..
            } => {
                let has_gif = formats.iter().any(|f| matches!(f, OutputFormat::Gif));
                let has_png = formats.iter().any(|f| matches!(f, OutputFormat::Png));

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
                let fd = frame_duration.unwrap_or(100);

                let captured_frames = capture::web::capture(
                    &full_url,
                    vp,
                    actions.as_deref().unwrap_or(&[]),
                    fd,
                )
                .await?;

                info!("captured {} web frames", captured_frames.len());

                if has_gif && !captured_frames.is_empty() {
                    let gif_path = output_dir.join(format!("{scene_name}.gif"));
                    crate::convert::gif::frames_to_gif(&captured_frames, &gif_path)?;
                    files.push(gif_path.display().to_string());
                }

                if has_png && !captured_frames.is_empty() {
                    let png_path = output_dir.join(format!("{scene_name}.png"));
                    write_last_frame_png(&captured_frames, &png_path)?;
                    files.push(png_path.display().to_string());
                }

                for format in formats {
                    match format {
                        OutputFormat::Mp4 => {
                            warn!("MP4 output requires ffmpeg in PATH - skipping for now");
                        }
                        _ => {}
                    }
                }
            }
            SceneConfig::Screen { .. } => {
                for format in formats {
                    match format {
                        OutputFormat::Png => {
                            let path = output_dir.join(format!("{scene_name}.png"));
                            capture_screen_scene(scene, &path).await?;
                            files.push(path.display().to_string());
                        }
                        OutputFormat::Gif => {
                            warn!("GIF output for screen scenes not yet supported");
                        }
                        OutputFormat::Mp4 => {
                            warn!("MP4 output requires ffmpeg in PATH - skipping for now");
                        }
                    }
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

/// Write the last frame of a capture as a PNG file.
fn write_last_frame_png(frames: &[CapturedFrame], output_path: &Path) -> Result<()> {
    let last = frames
        .last()
        .context("no frames to write")?;
    std::fs::write(output_path, &last.png_data)
        .with_context(|| format!("failed to write {}", output_path.display()))?;
    Ok(())
}

async fn capture_screen_scene(
    scene: &SceneConfig,
    output_path: &Path,
) -> Result<()> {
    match scene {
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
        _ => unreachable!("only screen scenes should be passed here"),
    }
}
