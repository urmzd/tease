use anyhow::{Context, Result};
use std::io::Cursor;
use tracing::{debug, info, warn};

use crate::backend::CaptureBackend;
use crate::types::{CapturedFrame, Interaction, Region};

pub struct ScreenBackend {
    display: Option<u32>,
    window: Option<String>,
    region: Option<Region>,
    frame_duration: u64,
    setup_command: Option<String>,
    delay: Option<u64>,
    title: Option<String>,
    theme: String,
    /// PID of the window matched after setup, so we can kill it on teardown.
    captured_pid: Option<u32>,
}

impl ScreenBackend {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        display: Option<u32>,
        window: Option<String>,
        region: Option<Region>,
        frame_duration: u64,
        setup_command: Option<String>,
        delay: Option<u64>,
        title: Option<String>,
        theme: String,
    ) -> Self {
        Self {
            display,
            window,
            region,
            frame_duration,
            setup_command,
            delay,
            title,
            theme,
            captured_pid: None,
        }
    }

    fn capture_image(&self) -> Result<image::RgbaImage> {
        let img = if let Some(ref query) = self.window {
            capture_window(query)?
        } else {
            capture_monitor(self.display)?
        };

        if let Some(ref r) = self.region {
            let sub = image::DynamicImage::ImageRgba8(img).crop_imm(r.x, r.y, r.width, r.height);
            Ok(match sub {
                image::DynamicImage::ImageRgba8(img) => img,
                other => other.to_rgba8(),
            })
        } else {
            Ok(img)
        }
    }

    fn capture_png_bytes(&self) -> Result<Vec<u8>> {
        let img = self.capture_image()?;
        let mut buf = Vec::new();
        img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
            .context("failed to encode PNG")?;
        Ok(buf)
    }

    async fn capture_framed(&self) -> Result<Vec<u8>> {
        let raw = self.capture_png_bytes()?;
        match crate::chrome_frame::render_with_chrome_frame(
            &raw,
            self.title.as_deref(),
            &self.theme,
        )
        .await
        {
            Ok(framed) => Ok(framed),
            Err(e) => {
                warn!("chrome framing failed, using raw screenshot: {e:#}");
                Ok(raw)
            }
        }
    }
}

#[async_trait::async_trait]
impl CaptureBackend for ScreenBackend {
    fn mode_name(&self) -> &'static str {
        "screen"
    }

    async fn setup(&mut self) -> Result<()> {
        if let Some(ref cmd) = self.setup_command {
            info!("running setup: {cmd}");
            std::process::Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .status()
                .context("setup command failed")?;
        }
        if let Some(ms) = self.delay {
            tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
        }
        // Record the PID of the matched window so we can close it on teardown
        if let Some(ref query) = self.window {
            if let Some(pid) = find_window_pid(query) {
                debug!("tracked window PID {} for teardown", pid);
                self.captured_pid = Some(pid);
            }
        }
        Ok(())
    }

    async fn execute(&mut self, interaction: &Interaction) -> Result<Vec<CapturedFrame>> {
        match interaction {
            Interaction::Snapshot { .. } => {
                let png_data = self.capture_framed().await?;
                Ok(vec![CapturedFrame {
                    png_data,
                    duration_ms: self.frame_duration,
                }])
            }
            Interaction::Wait { duration } => {
                tokio::time::sleep(std::time::Duration::from_millis(*duration)).await;
                Ok(vec![])
            }
            other => {
                debug!(
                    "skipping unsupported interaction: {:?} ({})",
                    other,
                    self.mode_name()
                );
                Ok(vec![])
            }
        }
    }

    async fn snapshot(&mut self) -> Result<CapturedFrame> {
        let png_data = self.capture_framed().await?;
        Ok(CapturedFrame {
            png_data,
            duration_ms: self.frame_duration,
        })
    }

    async fn teardown(&mut self) -> Result<()> {
        if let Some(pid) = self.captured_pid.take() {
            info!("closing captured window (PID {})", pid);
            #[cfg(unix)]
            {
                unsafe {
                    libc::kill(pid as i32, libc::SIGTERM);
                }
            }
            #[cfg(windows)]
            {
                std::process::Command::new("taskkill")
                    .args(["/PID", &pid.to_string(), "/F"])
                    .status()
                    .ok();
            }
        }
        Ok(())
    }
}

fn find_window_pid(query: &str) -> Option<u32> {
    let windows = xcap::Window::all().ok()?;
    let query_lower = query.to_lowercase();
    let win = windows
        .iter()
        .find(|w| {
            w.title()
                .map(|t| t.to_lowercase().contains(&query_lower))
                .unwrap_or(false)
        })
        .or_else(|| {
            windows.iter().find(|w| {
                w.app_name()
                    .map(|n| n.to_lowercase().contains(&query_lower))
                    .unwrap_or(false)
            })
        })?;
    win.pid().ok()
}

fn capture_monitor(display: Option<u32>) -> Result<image::RgbaImage> {
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

    monitor.capture_image().context("failed to capture screen")
}

fn capture_window(query: &str) -> Result<image::RgbaImage> {
    let windows = xcap::Window::all().context("failed to enumerate windows")?;
    let query_lower = query.to_lowercase();

    let win = windows
        .iter()
        .find(|w| {
            w.title()
                .map(|t| t.to_lowercase().contains(&query_lower))
                .unwrap_or(false)
        })
        .or_else(|| {
            windows.iter().find(|w| {
                w.app_name()
                    .map(|n| n.to_lowercase().contains(&query_lower))
                    .unwrap_or(false)
            })
        })
        .with_context(|| format!("no window matching '{query}' found"))?;

    win.capture_image()
        .context("failed to capture window image")
}
