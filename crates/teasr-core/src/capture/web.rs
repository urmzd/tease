use anyhow::{Context, Result};
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;
use futures::StreamExt;
use std::path::Path;
use std::time::Duration;

use crate::types::{ActionType, CaptureAction, ViewportConfig};

/// Capture a web page screenshot via CDP.
pub async fn capture(
    page_url: &str,
    viewport: &ViewportConfig,
    actions: &[CaptureAction],
    output_path: &Path,
) -> Result<()> {
    let config = BrowserConfig::builder()
        .window_size(viewport.width, viewport.height)
        .no_sandbox()
        .build()
        .map_err(|e| anyhow::anyhow!("browser config error: {e}"))?;

    let (mut browser, mut handler) = Browser::launch(config)
        .await
        .context("failed to launch browser")?;

    let handle = tokio::spawn(async move {
        while let Some(_event) = handler.next().await {}
    });

    let page = browser
        .new_page("about:blank")
        .await
        .context("failed to create page")?;

    // Navigate and wait with timeout
    page.goto(page_url).await.context("navigation failed")?;
    tokio::time::sleep(Duration::from_millis(1000)).await;

    // Execute actions
    for action in actions {
        execute_action(&page, action).await?;
    }

    let screenshot = page
        .screenshot(
            chromiumoxide::page::ScreenshotParams::builder()
                .format(CaptureScreenshotFormat::Png)
                .full_page(true)
                .build(),
        )
        .await
        .context("failed to take screenshot")?;

    std::fs::write(output_path, &screenshot)
        .with_context(|| format!("failed to write {}", output_path.display()))?;

    browser.close().await.ok();
    handle.await.ok();

    Ok(())
}

async fn execute_action(
    page: &chromiumoxide::Page,
    action: &CaptureAction,
) -> Result<()> {
    if let Some(delay) = action.delay {
        tokio::time::sleep(Duration::from_millis(delay)).await;
    }

    match action.action_type {
        ActionType::Click => {
            if let Some(sel) = &action.selector {
                page.find_element(sel)
                    .await
                    .context("element not found")?
                    .click()
                    .await
                    .context("click failed")?;
            }
        }
        ActionType::Hover => {
            if let Some(sel) = &action.selector {
                page.find_element(sel)
                    .await
                    .context("element not found")?
                    .click()
                    .await
                    .context("hover failed")?;
            }
        }
        ActionType::ScrollTo => {
            if let Some(sel) = &action.selector {
                let js = format!(
                    "document.querySelector('{}').scrollIntoView({{behavior:'smooth'}})",
                    sel.replace('\'', "\\'")
                );
                page.evaluate(js).await.context("scroll failed")?;
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
        ActionType::Wait => {
            let ms = action.delay.unwrap_or(1000);
            tokio::time::sleep(Duration::from_millis(ms)).await;
        }
        ActionType::Screenshot => {
            // Inline screenshot during action sequence
        }
    }

    Ok(())
}
