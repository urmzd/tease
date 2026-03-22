use anyhow::{Context, Result};
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;
use futures::StreamExt;
use std::path::PathBuf;
use std::time::Duration;
use tokio::task::JoinHandle;

use crate::backend::CaptureBackend;
use crate::types::{CapturedFrame, Interaction, ViewportConfig};

/// Capture backend for local files (HTML, PDF, SVG, etc.) rendered via headless Chrome.
pub struct FileBackend {
    path: PathBuf,
    viewport: ViewportConfig,
    frame_duration: u64,
    page_number: u32,
    browser: Option<Browser>,
    page: Option<chromiumoxide::Page>,
    handler: Option<JoinHandle<()>>,
}

impl FileBackend {
    pub fn new(
        path: impl Into<PathBuf>,
        viewport: ViewportConfig,
        frame_duration: u64,
        page_number: u32,
    ) -> Self {
        Self {
            path: path.into(),
            viewport,
            frame_duration,
            page_number,
            browser: None,
            page: None,
            handler: None,
        }
    }

    fn is_pdf(&self) -> bool {
        self.path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("pdf"))
    }

    async fn take_screenshot(&self) -> Result<Vec<u8>> {
        let page = self.page.as_ref().unwrap();
        page.screenshot(
            chromiumoxide::page::ScreenshotParams::builder()
                .format(CaptureScreenshotFormat::Png)
                .full_page(false)
                .build(),
        )
        .await
        .context("failed to take screenshot")
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

        let mut config_builder = BrowserConfig::builder()
            .window_size(self.viewport.width, self.viewport.height)
            .no_sandbox();

        // Disable Chrome's PDF viewer extension so PDFs render as page content
        if self.is_pdf() {
            config_builder = config_builder
                .arg("--disable-extensions")
                .arg("--disable-plugins");
        }

        let config = config_builder
            .build()
            .map_err(|e| anyhow::anyhow!("browser config error: {e}"))?;

        let (browser, mut handler) = Browser::launch(config)
            .await
            .context("failed to launch browser")?;

        let handle = tokio::spawn(async move {
            while let Some(_event) = handler.next().await {}
        });

        let page = browser
            .new_page("about:blank")
            .await
            .context("failed to create page")?;

        if self.is_pdf() {
            // Navigate to PDF page via fragment
            let url = if self.page_number > 1 {
                format!("{}#page={}", file_url, self.page_number)
            } else {
                file_url
            };
            page.goto(&url).await.context("navigation failed")?;
            // PDFs need extra time to render
            tokio::time::sleep(Duration::from_millis(2000)).await;
        } else {
            page.goto(&file_url).await.context("navigation failed")?;
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }

        self.browser = Some(browser);
        self.page = Some(page);
        self.handler = Some(handle);

        Ok(())
    }

    async fn execute(&mut self, interaction: &Interaction) -> Result<Vec<CapturedFrame>> {
        match interaction {
            Interaction::Wait { duration } => {
                tokio::time::sleep(Duration::from_millis(*duration)).await;
                Ok(vec![])
            }
            Interaction::Snapshot { .. } => {
                let png_data = self.take_screenshot().await?;
                Ok(vec![CapturedFrame {
                    png_data,
                    duration_ms: self.frame_duration,
                }])
            }
            _ => Ok(vec![]),
        }
    }

    async fn snapshot(&mut self) -> Result<CapturedFrame> {
        let png_data = self.take_screenshot().await?;
        Ok(CapturedFrame {
            png_data,
            duration_ms: self.frame_duration,
        })
    }

    async fn teardown(&mut self) -> Result<()> {
        if let Some(mut browser) = self.browser.take() {
            browser.close().await.ok();
        }
        if let Some(handle) = self.handler.take() {
            handle.await.ok();
        }
        Ok(())
    }
}
