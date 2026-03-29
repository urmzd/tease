use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::time::Instant;
use tempfile::NamedTempFile;
use tracing::debug;

use crate::backend::CaptureBackend;
use crate::capture;
use crate::render;
use crate::server::ManagedServer;
use crate::types::{
    CaptureResult, CapturedFrame, FontConfig, OutputFormat, ResolvedConfig, SceneConfig,
    ViewportConfig,
};
use crate::ui;

/// Build a backend for the given scene config.
///
/// Returns the backend and an optional temp file that must outlive the capture
/// (used when markdown is rendered to a temporary HTML file).
fn build_backend(
    scene: &SceneConfig,
    global_viewport: &ViewportConfig,
    server: Option<&crate::types::ServerConfig>,
    global_frame_duration_ms: u64,
    global_font: &FontConfig,
) -> Result<(Box<dyn CaptureBackend>, Option<NamedTempFile>)> {
    let default_fd = global_frame_duration_ms;
    let result = match scene {
        SceneConfig::Terminal {
            theme,
            cols,
            rows,
            name,
            cwd,
            font,
            frame_duration,
            ..
        } => {
            let effective_font = font.as_ref().unwrap_or(global_font);
            let backend: Box<dyn CaptureBackend> = Box::new(capture::terminal::TerminalBackend::new(
                cols.unwrap_or(80),
                *rows,
                theme.as_deref().unwrap_or("dracula"),
                name.clone(),
                frame_duration.unwrap_or(default_fd),
                cwd.clone(),
                Some(effective_font.family.clone()),
                Some(effective_font.size),
            ));
            (backend, None)
        }
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
            let backend: Box<dyn CaptureBackend> = Box::new(capture::web::WebBackend::new(
                full_url,
                vp,
                frame_duration.unwrap_or(default_fd),
                full_page.unwrap_or(false),
            ));
            (backend, None)
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
        } => {
            let backend: Box<dyn CaptureBackend> = Box::new(capture::screen::ScreenBackend::new(
                *display,
                window.clone(),
                region.clone(),
                frame_duration.unwrap_or(default_fd),
                setup.clone(),
                *delay,
                title.clone(),
                theme.clone(),
            ));
            (backend, None)
        }
        SceneConfig::File {
            path,
            viewport,
            frame_duration,
            page,
            ..
        } => {
            let vp = viewport.as_ref().unwrap_or(global_viewport).clone();
            let backend: Box<dyn CaptureBackend> = Box::new(capture::file::FileBackend::new(
                path,
                vp,
                frame_duration.unwrap_or(default_fd),
                *page,
                false,
            ));
            (backend, None)
        }
        SceneConfig::Markdown {
            path,
            viewport,
            frame_duration,
            flavor,
            theme,
            stylesheet,
            template,
            full_page,
            ..
        } => {
            let vp = viewport.as_ref().unwrap_or(global_viewport).clone();
            let tmp = render::markdown::render_to_html(
                Path::new(path),
                flavor,
                theme,
                stylesheet.as_deref().map(Path::new),
                template.as_deref().map(Path::new),
                vp.width,
            )?;
            let backend: Box<dyn CaptureBackend> = Box::new(capture::file::FileBackend::new(
                tmp.path(),
                vp,
                frame_duration.unwrap_or(default_fd),
                1,
                *full_page,
            ));
            (backend, Some(tmp))
        }
    };
    Ok(result)
}

/// Run all scenes in order and return capture results.
pub async fn run(config: &ResolvedConfig) -> Result<Vec<CaptureResult>> {
    let output_dir = PathBuf::from(&config.output.dir);
    std::fs::create_dir_all(&output_dir)
        .with_context(|| format!("failed to create output dir: {}", output_dir.display()))?;

    // Load custom font if configured
    if let Some(ref path) = config.font.path {
        teasr_term_render::load_extra_font(std::path::Path::new(path))?;
    }
    // Also load per-scene custom fonts
    for scene in &config.scenes {
        if let SceneConfig::Terminal { font: Some(ref f), .. } = scene {
            if let Some(ref path) = f.path {
                teasr_term_render::load_extra_font(std::path::Path::new(path))?;
            }
        }
    }

    // Start server if configured
    let _server = match &config.server {
        Some(server_config) => Some(ManagedServer::start(server_config).await?),
        None => None,
    };

    let mut results = Vec::new();

    for (i, scene) in config.scenes.iter().enumerate() {
        let scene_name = scene.name().to_string();
        let label = format!(
            "[{}/{}] capturing: {} ({})",
            i + 1,
            config.scenes.len(),
            scene_name,
            scene.scene_type()
        );
        let pb = ui::spinner(&label);
        let start = Instant::now();

        match capture_scene(
            scene,
            &config.output,
            &config.viewport,
            config.server.as_ref(),
            &output_dir,
            config.frame_duration_ms,
            config.scene_timeout,
            &config.font,
            &pb,
        )
        .await
        {
            Ok(result) => {
                let elapsed = start.elapsed();
                let detail = format!("{} frames in {:.1}s", result.files.len(), elapsed.as_secs_f64());
                ui::spinner_done(&pb, Some(&detail));
                results.push(result);
            }
            Err(e) => {
                pb.finish_and_clear();
                ui::error(&format!("scene '{}' failed: {e:#}", scene_name));
            }
        }
    }

    if results.is_empty() && !config.scenes.is_empty() {
        anyhow::bail!("all scenes failed");
    }

    ui::phase_ok(
        &format!("{}/{} scenes captured successfully", results.len(), config.scenes.len()),
        None,
    );
    Ok(results)
}

#[allow(clippy::too_many_arguments)]
async fn capture_scene(
    scene: &SceneConfig,
    output_config: &crate::types::OutputConfig,
    global_viewport: &ViewportConfig,
    server: Option<&crate::types::ServerConfig>,
    output_dir: &Path,
    global_frame_duration_ms: u64,
    seconds: f64,
    global_font: &FontConfig,
    pb: &indicatif::ProgressBar,
) -> Result<CaptureResult> {
    let scene_name = scene.name().to_string();
    let formats = scene
        .formats()
        .as_ref()
        .unwrap_or(&output_config.formats);

    let (mut backend, _tmp_file) = build_backend(scene, global_viewport, server, global_frame_duration_ms, global_font)?;
    pb.set_message(format!("{scene_name}: setting up"));
    backend.setup().await?;

    let capture_fut = async {
        let mut frames = Vec::new();

        // Intro splash for terminal scenes
        if let SceneConfig::Terminal {
            intro: Some(ref splash),
            cols,
            rows,
            theme,
            font,
            ..
        } = scene
        {
            pb.set_message(format!("{scene_name}: rendering intro"));
            let splash_frames = render_splash(
                splash,
                cols.unwrap_or(80),
                rows.unwrap_or(24),
                theme.as_deref().unwrap_or("dracula"),
                font.as_ref().unwrap_or(global_font),
            )?;
            frames.extend(splash_frames);
        }

        // Initial frame for terminal (shows prompt)
        if matches!(scene, SceneConfig::Terminal { .. }) {
            frames.push(backend.snapshot().await?);
        }

        let interactions = scene.interactions();
        for (j, step) in interactions.iter().enumerate() {
            let step_label = interaction_label(&step.interaction);
            pb.set_message(format!(
                "{scene_name}: step {}/{} {step_label}",
                j + 1,
                interactions.len()
            ));
            let step_frames = backend.execute(&step.interaction).await?;
            if !step.hidden {
                frames.extend(step_frames);
            }
        }

        // Fallback: at least one frame
        if frames.is_empty() {
            frames.push(backend.snapshot().await?);
        }

        // Outro splash for terminal scenes
        if let SceneConfig::Terminal {
            outro: Some(ref splash),
            cols,
            rows,
            theme,
            font,
            ..
        } = scene
        {
            pb.set_message(format!("{scene_name}: rendering outro"));
            let splash_frames = render_splash(
                splash,
                cols.unwrap_or(80),
                rows.unwrap_or(24),
                theme.as_deref().unwrap_or("dracula"),
                font.as_ref().unwrap_or(global_font),
            )?;
            frames.extend(splash_frames);
        }

        Ok::<_, anyhow::Error>(frames)
    };

    let timeout = std::time::Duration::from_secs_f64(seconds);
    let frames = match tokio::time::timeout(timeout, capture_fut).await {
        Ok(result) => result?,
        Err(_) => anyhow::bail!("scene '{}' timed out after {:.1}s", scene_name, seconds),
    };

    backend.teardown().await?;

    debug!("captured {} frames", frames.len());

    pb.set_message(format!("{scene_name}: encoding outputs"));
    let files = write_outputs(&frames, &scene_name, formats, output_dir)?;

    Ok(CaptureResult { scene_name, files })
}

/// Human-readable label for an interaction step.
fn interaction_label(interaction: &crate::types::Interaction) -> String {
    use crate::types::Interaction;
    match interaction {
        Interaction::Type { text, .. } => {
            let preview: String = text.chars().take(30).collect();
            let suffix = if text.len() > 30 { "…" } else { "" };
            format!("typing \"{preview}{suffix}\"")
        }
        Interaction::Key { key } => format!("pressing {key}"),
        Interaction::Wait { duration, .. } => format!("waiting {duration}ms"),
        Interaction::Click { selector, .. } => {
            format!("clicking {}", selector.as_deref().unwrap_or("page"))
        }
        Interaction::Hover { selector, .. } => {
            format!("hovering {}", selector.as_deref().unwrap_or("page"))
        }
        Interaction::ScrollTo { selector, .. } => {
            format!("scrolling to {}", selector.as_deref().unwrap_or("top"))
        }
        Interaction::Snapshot { name, .. } => {
            format!("snapshot{}", name.as_ref().map(|n| format!(" ({n})")).unwrap_or_default())
        }
    }
}

/// Render splash frames from a SplashConfig.
fn render_splash(
    splash: &crate::types::SplashConfig,
    cols: usize,
    rows: usize,
    theme: &str,
    font: &FontConfig,
) -> Result<Vec<CapturedFrame>> {
    let opts = teasr_term_render::RenderOptions {
        theme_name: theme,
        title: None,
        font_family: Some(&font.family),
        font_size: Some(font.size),
    };

    let png_data = if let Some(ref text) = splash.text {
        teasr_term_render::splash::render_text_splash(text, cols, rows, splash.center, &opts)?
    } else if let Some(ref file) = splash.file {
        let content = std::fs::read(file)
            .with_context(|| format!("failed to read splash file: {file}"))?;
        teasr_term_render::splash::render_ansi_splash(&content, cols, rows, splash.center, &opts)?
    } else if let Some(ref image) = splash.image {
        let data = std::fs::read(image)
            .with_context(|| format!("failed to read splash image: {image}"))?;
        teasr_term_render::splash::render_image_splash(&data, cols, rows, splash.center, &opts)?
    } else {
        return Ok(vec![]);
    };

    Ok(vec![CapturedFrame {
        png_data,
        duration_ms: splash.duration,
    }])
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
                ui::warn("MP4 output requires ffmpeg in PATH - skipping for now");
            }
            _ => {}
        }
    }

    Ok(files)
}
