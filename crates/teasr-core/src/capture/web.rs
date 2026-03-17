use anyhow::{Context, Result};
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::input::{
    DispatchMouseEventParams, DispatchMouseEventType,
};
use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;
use futures::StreamExt;
use std::time::Duration;
use tokio::task::JoinHandle;
use crate::backend::CaptureBackend;
use crate::types::{CapturedFrame, Interaction, ViewportConfig};

pub struct WebBackend {
    url: String,
    viewport: ViewportConfig,
    frame_duration: u64,
    full_page: bool,
    browser: Option<Browser>,
    page: Option<chromiumoxide::Page>,
    handler: Option<JoinHandle<()>>,
}

impl WebBackend {
    pub fn new(url: String, viewport: ViewportConfig, frame_duration: u64, full_page: bool) -> Self {
        Self {
            url,
            viewport,
            frame_duration,
            full_page,
            browser: None,
            page: None,
            handler: None,
        }
    }

    async fn take_screenshot(&self) -> Result<Vec<u8>> {
        let page = self.page.as_ref().unwrap();
        page.screenshot(
            chromiumoxide::page::ScreenshotParams::builder()
                .format(CaptureScreenshotFormat::Png)
                .full_page(self.full_page)
                .build(),
        )
        .await
        .context("failed to take screenshot")
    }
}

#[async_trait::async_trait]
impl CaptureBackend for WebBackend {
    fn mode_name(&self) -> &'static str {
        "web"
    }

    async fn setup(&mut self) -> Result<()> {
        let config = BrowserConfig::builder()
            .window_size(self.viewport.width, self.viewport.height)
            .no_sandbox()
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

        page.goto(&self.url).await.context("navigation failed")?;
        tokio::time::sleep(Duration::from_millis(1000)).await;

        self.browser = Some(browser);
        self.page = Some(page);
        self.handler = Some(handle);

        Ok(())
    }

    async fn execute(&mut self, interaction: &Interaction) -> Result<Vec<CapturedFrame>> {
        let page = self.page.as_ref().unwrap();

        match interaction {
            Interaction::Click { selector } => {
                if let Some(sel) = selector {
                    page.find_element(sel)
                        .await
                        .context("element not found")?
                        .click()
                        .await
                        .context("click failed")?;
                }
                Ok(vec![])
            }
            Interaction::Hover { selector } => {
                if let Some(sel) = selector {
                    let js = format!(
                        "(() => {{ const r = document.querySelector('{}').getBoundingClientRect(); return [r.x + r.width/2, r.y + r.height/2]; }})()",
                        sel.replace('\'', "\\'")
                    );
                    let coords: Vec<f64> = page
                        .evaluate(js)
                        .await
                        .context("failed to get element position")?
                        .into_value()
                        .context("failed to parse coordinates")?;
                    page.execute(
                        DispatchMouseEventParams::builder()
                            .r#type(DispatchMouseEventType::MouseMoved)
                            .x(coords[0])
                            .y(coords[1])
                            .build()
                            .unwrap(),
                    )
                    .await
                    .context("hover dispatch failed")?;
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
                    page.evaluate(js).await.context("scroll failed")?;
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
                Ok(vec![])
            }
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
            Interaction::Type { text, speed } => {
                let delay = speed.unwrap_or(50);
                for ch in text.chars() {
                    page.evaluate(format!(
                        "document.activeElement && document.activeElement.dispatchEvent(new KeyboardEvent('keypress', {{key: '{}'}}))",
                        ch
                    ))
                    .await
                    .ok();
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                }
                Ok(vec![])
            }
            Interaction::Key { key } => {
                page.evaluate(format!(
                    "document.activeElement && document.activeElement.dispatchEvent(new KeyboardEvent('keydown', {{key: '{}'}}))",
                    key
                ))
                .await
                .ok();
                Ok(vec![])
            }
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
