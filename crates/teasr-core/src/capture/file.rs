use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::backend::CaptureBackend;
use crate::browser::{self, BrowserEngine, BrowserPage, LaunchOptions};
use crate::capture::chrome::{install_idle_tracker, page_has_activity};
use crate::capture::wait_for_idle;
use crate::types::{CapturedFrame, Interaction, ViewportConfig};

/// Capture backend for local files (HTML, PDF, SVG, etc.) rendered via headless browser.
pub struct FileBackend {
    path: PathBuf,
    viewport: ViewportConfig,
    frame_duration: u64,
    page_number: u32,
    full_page: bool,
    browser: Option<Box<dyn BrowserEngine>>,
    page: Option<Box<dyn BrowserPage>>,
}

impl FileBackend {
    pub fn new(
        path: impl Into<PathBuf>,
        viewport: ViewportConfig,
        frame_duration: u64,
        page_number: u32,
        full_page: bool,
    ) -> Self {
        Self {
            path: path.into(),
            viewport,
            frame_duration,
            page_number,
            full_page,
            browser: None,
            page: None,
        }
    }

    fn is_pdf(&self) -> bool {
        self.path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("pdf"))
    }
}

#[async_trait::async_trait]
impl CaptureBackend for FileBackend {
    fn mode_name(&self) -> &'static str {
        "file"
    }

    async fn setup(&mut self) -> Result<()> {
        let abs_path = std::fs::canonicalize(&self.path)
            .with_context(|| format!("file not found: {}", self.path.display()))?;

        let file_url = format!("file://{}", abs_path.display());

        let mut opts = LaunchOptions::new(self.viewport.width, self.viewport.height);
        if self.is_pdf() {
            opts = opts.arg("--disable-extensions").arg("--disable-plugins");
        }

        let browser = browser::launch(opts).await?;
        let page = browser.new_page("about:blank").await?;

        if self.is_pdf() {
            let url = if self.page_number > 1 {
                format!(
                    "{}#page={}&toolbar=0&navpanes=0&scrollbar=0",
                    file_url, self.page_number
                )
            } else {
                format!("{}#toolbar=0&navpanes=0&scrollbar=0", file_url)
            };
            page.goto(&url).await?;
            install_idle_tracker(&*page).await;
            wait_for_idle(10000, 500, 50, || page_has_activity(&*page)).await;
        } else {
            page.goto(&file_url).await?;
            install_idle_tracker(&*page).await;
            wait_for_idle(5000, 500, 50, || page_has_activity(&*page)).await;
        }

        self.browser = Some(browser);
        self.page = Some(page);

        Ok(())
    }

    async fn execute(&mut self, interaction: &Interaction) -> Result<Vec<CapturedFrame>> {
        match interaction {
            Interaction::Wait {
                duration,
                idle_timeout,
            } => {
                let page = self.page.as_ref().unwrap();
                install_idle_tracker(&**page).await;
                wait_for_idle(*duration, *idle_timeout, 50, || page_has_activity(&**page)).await;
                Ok(vec![])
            }
            Interaction::Snapshot { .. } => {
                let page = self.page.as_ref().unwrap();
                let png_data = page.screenshot(self.full_page).await?;
                Ok(vec![CapturedFrame {
                    png_data,
                    duration_ms: self.frame_duration,
                }])
            }
            _ => Ok(vec![]),
        }
    }

    async fn snapshot(&mut self) -> Result<CapturedFrame> {
        let page = self.page.as_ref().unwrap();
        let png_data = page.screenshot(self.full_page).await?;
        Ok(CapturedFrame {
            png_data,
            duration_ms: self.frame_duration,
        })
    }

    async fn teardown(&mut self) -> Result<()> {
        self.page = None;
        if let Some(mut browser) = self.browser.take() {
            browser.close().await;
        }
        Ok(())
    }
}
