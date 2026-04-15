//! Chromium browser engine backed by chromiumoxide.

use anyhow::{Context, Result};
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::input::{
    DispatchMouseEventParams, DispatchMouseEventType,
};
use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;
use futures::StreamExt;
use serde_json::Value;
use tokio::task::JoinHandle;

use super::{BrowserEngine, BrowserPage, LaunchOptions};

pub struct ChromiumEngine {
    inner: Browser,
    handler: Option<JoinHandle<()>>,
}

struct ChromiumPage {
    inner: chromiumoxide::Page,
}

impl ChromiumEngine {
    pub async fn launch(opts: LaunchOptions) -> Result<Self> {
        let mut builder = BrowserConfig::builder()
            .window_size(opts.width, opts.height)
            .no_sandbox();

        for arg in &opts.extra_args {
            builder = builder.arg(arg.as_str());
        }

        let config = builder
            .build()
            .map_err(|e| anyhow::anyhow!("browser config error: {e}"))?;

        let (browser, mut handler) = Browser::launch(config)
            .await
            .context("failed to launch browser")?;

        let handle = tokio::spawn(async move { while let Some(_event) = handler.next().await {} });

        Ok(Self {
            inner: browser,
            handler: Some(handle),
        })
    }
}

#[async_trait::async_trait]
impl BrowserEngine for ChromiumEngine {
    async fn new_page(&self, url: &str) -> Result<Box<dyn BrowserPage>> {
        let page = self
            .inner
            .new_page(url)
            .await
            .context("failed to create page")?;
        Ok(Box::new(ChromiumPage { inner: page }))
    }

    async fn close(&mut self) {
        self.inner.close().await.ok();
        if let Some(handle) = self.handler.take() {
            handle.await.ok();
        }
    }
}

#[async_trait::async_trait]
impl BrowserPage for ChromiumPage {
    async fn goto(&self, url: &str) -> Result<()> {
        self.inner.goto(url).await.context("navigation failed")?;
        Ok(())
    }

    async fn evaluate_value(&self, js: &str) -> Result<Value> {
        self.inner
            .evaluate(js)
            .await
            .context("JS evaluation failed")?
            .into_value::<Value>()
            .context("failed to extract JS result")
    }

    async fn try_evaluate_value(&self, js: &str) -> Option<Value> {
        self.inner
            .evaluate(js)
            .await
            .ok()
            .and_then(|v| v.into_value::<Value>().ok())
    }

    async fn execute(&self, js: &str) {
        self.inner.evaluate(js).await.ok();
    }

    async fn screenshot(&self, full_page: bool) -> Result<Vec<u8>> {
        self.inner
            .screenshot(
                chromiumoxide::page::ScreenshotParams::builder()
                    .format(CaptureScreenshotFormat::Png)
                    .full_page(full_page)
                    .build(),
            )
            .await
            .context("failed to take screenshot")
    }

    async fn find_and_click(&self, selector: &str) -> Result<()> {
        self.inner
            .find_element(selector)
            .await
            .context("element not found")?
            .click()
            .await
            .context("click failed")?;
        Ok(())
    }

    async fn mouse_move(&self, x: f64, y: f64) -> Result<()> {
        self.inner
            .execute(
                DispatchMouseEventParams::builder()
                    .r#type(DispatchMouseEventType::MouseMoved)
                    .x(x)
                    .y(y)
                    .build()
                    .unwrap(),
            )
            .await
            .context("mouse move failed")?;
        Ok(())
    }
}
