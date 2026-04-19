use anyhow::{Context, Result};
use std::time::Duration;

use crate::backend::CaptureBackend;
use crate::browser::{self, BrowserEngine, BrowserPage, LaunchOptions};
use crate::capture::chrome::{install_idle_tracker, page_has_activity};
use crate::capture::wait_for_idle;
use crate::types::{CapturedFrame, Interaction, ViewportConfig};

pub struct WebBackend {
    url: String,
    viewport: ViewportConfig,
    frame_duration: u64,
    full_page: bool,
    browser: Option<Box<dyn BrowserEngine>>,
    page: Option<Box<dyn BrowserPage>>,
}

impl WebBackend {
    pub fn new(
        url: String,
        viewport: ViewportConfig,
        frame_duration: u64,
        full_page: bool,
    ) -> Self {
        Self {
            url,
            viewport,
            frame_duration,
            full_page,
            browser: None,
            page: None,
        }
    }

    fn is_pdf_url(&self) -> bool {
        self.url
            .split(['?', '#'])
            .next()
            .is_some_and(|base| base.to_ascii_lowercase().ends_with(".pdf"))
    }
}

#[async_trait::async_trait]
impl CaptureBackend for WebBackend {
    fn mode_name(&self) -> &'static str {
        "web"
    }

    async fn setup(&mut self) -> Result<()> {
        let mut opts = LaunchOptions::new(self.viewport.width, self.viewport.height);
        if self.is_pdf_url() {
            opts = opts.arg("--disable-extensions").arg("--disable-plugins");
        }
        let browser = browser::launch(opts).await?;
        let page = browser.new_page("about:blank").await?;

        page.goto(&self.url).await?;
        install_idle_tracker(&*page).await;
        wait_for_idle(5000, 500, 50, || page_has_activity(&*page)).await;

        self.browser = Some(browser);
        self.page = Some(page);

        Ok(())
    }

    async fn execute(&mut self, interaction: &Interaction) -> Result<Vec<CapturedFrame>> {
        let page = self.page.as_ref().unwrap();

        match interaction {
            Interaction::Click { selector } => {
                if let Some(sel) = selector {
                    page.find_and_click(sel).await?;
                }
                Ok(vec![])
            }
            Interaction::Hover { selector } => {
                if let Some(sel) = selector {
                    let js = format!(
                        "(() => {{ const r = document.querySelector('{}').getBoundingClientRect(); return [r.x + r.width/2, r.y + r.height/2]; }})()",
                        sel.replace('\'', "\\'")
                    );
                    let coords: Vec<f64> = browser::evaluate(&**page, &js)
                        .await
                        .context("failed to get element position")?;
                    page.mouse_move(coords[0], coords[1]).await?;
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
                Ok(vec![])
            }
            Interaction::ScrollTo { selector } => {
                if let Some(sel) = selector {
                    let js = format!(
                        "document.querySelector('{}').scrollIntoView({{behavior:'smooth'}})",
                        sel.replace('\'', "\\'")
                    );
                    page.execute(&js).await;
                }
                Ok(vec![])
            }
            Interaction::Wait { duration } => {
                tokio::time::sleep(Duration::from_millis(*duration)).await;
                Ok(vec![])
            }
            Interaction::Snapshot { .. } => {
                let png_data = page.screenshot(self.full_page).await?;
                Ok(vec![CapturedFrame {
                    png_data,
                    duration_ms: self.frame_duration,
                }])
            }
            Interaction::Type { text, speed } => {
                let delay = speed.unwrap_or(50);
                for ch in text.chars() {
                    page.execute(&format!(
                        "document.activeElement && document.activeElement.dispatchEvent(new KeyboardEvent('keypress', {{key: '{}'}}))",
                        ch
                    )).await;
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                }
                Ok(vec![])
            }
            Interaction::Key { key } => {
                page.execute(&format!(
                    "document.activeElement && document.activeElement.dispatchEvent(new KeyboardEvent('keydown', {{key: '{}'}}))",
                    key
                )).await;
                Ok(vec![])
            }
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
