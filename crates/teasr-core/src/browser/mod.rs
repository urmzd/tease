//! Browser engine abstraction.
//!
//! Defines [`BrowserEngine`] and [`BrowserPage`] traits that isolate capture
//! backends from any specific browser implementation. The default engine is
//! Chromium (via chromiumoxide) — swap it by providing a different
//! [`BrowserEngine`] impl and changing the [`launch`] factory.

mod chromium;

use anyhow::Result;
use serde::de::DeserializeOwned;
use serde_json::Value;

/// Options for launching a browser (engine-agnostic).
pub struct LaunchOptions {
    pub width: u32,
    pub height: u32,
    pub extra_args: Vec<String>,
}

impl LaunchOptions {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            extra_args: Vec::new(),
        }
    }

    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.extra_args.push(arg.into());
        self
    }
}

// ---------------------------------------------------------------------------
// Traits
// ---------------------------------------------------------------------------

/// A running browser instance that can open pages.
#[async_trait::async_trait]
pub trait BrowserEngine: Send {
    /// Open a new page/tab navigated to `url`.
    async fn new_page(&self, url: &str) -> Result<Box<dyn BrowserPage>>;

    /// Shut down the browser. After this call the engine is unusable.
    /// Uses `&mut self` (not `self`) for object-safety with `Box<dyn>`.
    async fn close(&mut self);
}

/// A single browser page/tab.
#[async_trait::async_trait]
pub trait BrowserPage: Send + Sync {
    /// Navigate to a URL and wait for the initial load.
    async fn goto(&self, url: &str) -> Result<()>;

    /// Evaluate JavaScript and return the raw JSON result.
    async fn evaluate_value(&self, js: &str) -> Result<Value>;

    /// Evaluate JavaScript, returning `None` on any failure.
    async fn try_evaluate_value(&self, js: &str) -> Option<Value>;

    /// Fire-and-forget JavaScript execution (result discarded).
    async fn execute(&self, js: &str);

    /// Capture a PNG screenshot of the page.
    async fn screenshot(&self, full_page: bool) -> Result<Vec<u8>>;

    /// Find an element by CSS selector and click it.
    async fn find_and_click(&self, selector: &str) -> Result<()>;

    /// Move the mouse cursor to absolute page coordinates.
    async fn mouse_move(&self, x: f64, y: f64) -> Result<()>;
}

// ---------------------------------------------------------------------------
// Typed convenience helpers (generic → not on the trait)
// ---------------------------------------------------------------------------

/// Evaluate JavaScript and deserialize the result into `T`.
pub async fn evaluate<T: DeserializeOwned>(page: &dyn BrowserPage, js: &str) -> Result<T> {
    let val = page.evaluate_value(js).await?;
    serde_json::from_value(val).map_err(|e| anyhow::anyhow!("deserialize failed: {e}"))
}

/// Evaluate JavaScript, returning `None` on failure or deserialization error.
pub async fn try_evaluate<T: DeserializeOwned>(page: &dyn BrowserPage, js: &str) -> Option<T> {
    page.try_evaluate_value(js)
        .await
        .and_then(|v| serde_json::from_value(v).ok())
}

// ---------------------------------------------------------------------------
// Default factory
// ---------------------------------------------------------------------------

/// Launch the default browser engine (Chromium).
pub async fn launch(opts: LaunchOptions) -> Result<Box<dyn BrowserEngine>> {
    Ok(Box::new(chromium::ChromiumEngine::launch(opts).await?))
}
