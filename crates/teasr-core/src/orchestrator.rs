use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::{error, info, warn};

use crate::backend::CaptureBackend;
use crate::capture;
use crate::server::ManagedServer;
use crate::types::{
    CaptureResult, CapturedFrame, OutputFormat, ResolvedConfig, SceneConfig, ViewportConfig,
};

/// Build a backend for the given scene config.
fn build_backend(
    scene: &SceneConfig,
    global_viewport: &ViewportConfig,
    server: Option<&crate::types::ServerConfig>,
    global_frame_duration_ms: u64,
) -> Box<dyn CaptureBackend> {
    let default_fd = global_frame_duration_ms;
    match scene {
        SceneConfig::Terminal {
            theme,
            cols,
            rows,
            name,
            frame_duration,
            ..
        } => Box::new(capture::terminal::TerminalBackend::new(
            cols.unwrap_or(80),
            *rows,
            theme.as_deref().unwrap_or("dracula"),
            name.clone(),
            frame_duration.unwrap_or(default_fd),
        )),
        SceneConfig::Web {
            url,
            viewport,
            frame_duration,
            full_page,
            ..
        } => {
            let vp = viewport.as_ref().unwrap_or(global_viewport).clone();
            let full_url = if let Some(srv) = server {
                if url.starts_with('/') {
                    format!("{}{}", srv.url.trim_end_matches('/'), url)
                } else {
                    url.clone()
                }
            } else {
                url.clone()
            };
            Box::new(capture::web::WebBackend::new(
                full_url,
                vp,
                frame_duration.unwrap_or(default_fd),
                full_page.unwrap_or(true),
            ))
        }
        SceneConfig::Screen {
            display,
            window,
            region,
            frame_duration,
            setup,
            delay,
            title,
            theme,
            ..
        } => Box::new(capture::screen::ScreenBackend::new(
            *display,
            window.clone(),
            region.clone(),
            frame_duration.unwrap_or(default_fd),
            setup.clone(),
            *delay,
            title.clone(),
            theme.clone(),
        )),
    }
}

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
        info!(
            "[{}/{}] capturing: {} ({})",
            i + 1,
            config.scenes.len(),
            scene_name,
            scene.scene_type()
        );

        match capture_scene(
            scene,
            &config.output,
            &config.viewport,
            config.server.as_ref(),
            &output_dir,
            config.frame_duration_ms,
            config.seconds,
        )
        .await
        {
            Ok(result) => results.push(result),
            Err(e) => error!("scene '{}' failed: {e:#}", scene_name),
        }
    }

    if results.is_empty() && !config.scenes.is_empty() {
        anyhow::bail!("all scenes failed");
    }

    info!(
        "{}/{} scenes captured successfully",
        results.len(),
        config.scenes.len()
    );
    Ok(results)
}

async fn capture_scene(
    scene: &SceneConfig,
    output_config: &crate::types::OutputConfig,
    global_viewport: &ViewportConfig,
    server: Option<&crate::types::ServerConfig>,
    output_dir: &Path,
    global_frame_duration_ms: u64,
    seconds: f64,
) -> Result<CaptureResult> {
    let scene_name = scene.name().to_string();
    let formats = scene
        .formats()
        .as_ref()
        .unwrap_or(&output_config.formats);

    let mut backend = build_backend(scene, global_viewport, server, global_frame_duration_ms);
    backend.setup().await?;

    let capture_fut = async {
        let mut frames = Vec::new();

        // Initial frame for terminal (shows prompt)
        if matches!(scene, SceneConfig::Terminal { .. }) {
            frames.push(backend.snapshot().await?);
        }

        for interaction in scene.interactions() {
            frames.extend(backend.execute(interaction).await?);
        }

        // Fallback: at least one frame
        if frames.is_empty() {
            frames.push(backend.snapshot().await?);
        }

        Ok::<_, anyhow::Error>(frames)
    };

    let timeout = std::time::Duration::from_secs_f64(seconds);
    let frames = match tokio::time::timeout(timeout, capture_fut).await {
        Ok(result) => result?,
        Err(_) => anyhow::bail!("scene '{}' timed out after {:.1}s", scene_name, seconds),
    };

    backend.teardown().await?;

    info!("captured {} frames", frames.len());

    let files = write_outputs(&frames, &scene_name, formats, output_dir)?;

    Ok(CaptureResult { scene_name, files })
}

/// Write frames to disk in the requested formats.
fn write_outputs(
    frames: &[CapturedFrame],
    scene_name: &str,
    formats: &[OutputFormat],
    output_dir: &Path,
) -> Result<Vec<String>> {
    let mut files = Vec::new();

    for format in formats {
        match format {
            OutputFormat::Gif(gif_config) if !frames.is_empty() => {
                let gif_path = output_dir.join(format!("{scene_name}.gif"));
                crate::convert::gif::frames_to_gif(frames, &gif_path, gif_config)?;
                files.push(gif_path.display().to_string());
            }
            OutputFormat::Png(_) if !frames.is_empty() => {
                let png_path = output_dir.join(format!("{scene_name}.png"));
                let last = frames.last().context("no frames to write")?;
                std::fs::write(&png_path, &last.png_data)
                    .with_context(|| format!("failed to write {}", png_path.display()))?;
                files.push(png_path.display().to_string());
            }
            OutputFormat::Mp4(_) => {
                warn!("MP4 output requires ffmpeg in PATH - skipping for now");
            }
            _ => {}
        }
    }

    Ok(files)
}
