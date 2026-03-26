use anyhow::{Context, Result};
use base64::Engine;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;
use futures::StreamExt;
use tracing::debug;

/// Render a PNG image inside macOS-style window chrome using headless Chrome.
///
/// Visual constants match `teasr-term-render/src/svg.rs` and `themes.rs`:
/// chrome height 40px, corner radius 10px, padding 16px, Dracula theme by default.
pub async fn render_with_chrome_frame(
    png_data: &[u8],
    title: Option<&str>,
    theme: &str,
) -> Result<Vec<u8>> {
    let img = image::load_from_memory(png_data).context("failed to decode PNG for framing")?;
    let img_w = img.width();
    let img_h = img.height();

    // Scale down for Chrome rendering if the image is too large
    const MAX_WIDTH: u32 = 1280;
    let scale = if img_w > MAX_WIDTH {
        MAX_WIDTH as f64 / img_w as f64
    } else {
        1.0
    };
    let render_w = (img_w as f64 * scale).round() as u32;
    let render_h = (img_h as f64 * scale).round() as u32;

    let (bg, chrome_bg, fg, btn_close, btn_min, btn_max) = match theme {
        "monokai" => ("#272822", "#1e1f1c", "#f8f8f2", "#f92672", "#f4bf75", "#a6e22e"),
        _ => ("#282a36", "#1e1f29", "#f8f8f2", "#ff5555", "#f1fa8c", "#50fa7b"),
    };

    let title_text = title.unwrap_or("Screen Capture");
    let b64 = base64::engine::general_purpose::STANDARD.encode(png_data);

    let html = include_str!("chrome_frame.html")
        .replace("__VP_W__", &(render_w + 32 + 80).to_string()) // content padding (16*2) + body padding (40*2)
        .replace("__VP_H__", &(render_h + 40 + 32 + 80).to_string()) // chrome + content padding + body padding
        .replace("__WIN_W__", &(render_w + 32).to_string()) // content padding
        .replace("__CHROME_BG__", chrome_bg)
        .replace("__BTN_CLOSE__", btn_close)
        .replace("__BTN_MIN__", btn_min)
        .replace("__BTN_MAX__", btn_max)
        .replace("__FG__", fg)
        .replace("__BG__", bg)
        .replace("__IMG_W__", &render_w.to_string())
        .replace("__IMG_H__", &render_h.to_string())
        .replace("__TITLE__", title_text)
        .replace("__B64__", &b64);

    let tmp = std::env::temp_dir().join(format!("teasr-frame-{}.html", std::process::id()));
    std::fs::write(&tmp, html.as_bytes()).context("failed to write temp HTML for chrome frame")?;
    let file_url = format!("file://{}", tmp.display());

    let vp_w = render_w + 32 + 80;
    let vp_h = render_h + 40 + 32 + 80;

    debug!("chrome frame: viewport {}x{}", vp_w, vp_h);

    let config = BrowserConfig::builder()
        .window_size(vp_w, vp_h)
        .no_sandbox()
        .build()
        .map_err(|e| anyhow::anyhow!("browser config error: {e}"))?;

    let (browser, mut handler) = Browser::launch(config)
        .await
        .context("failed to launch browser for chrome framing")?;

    let handle = tokio::spawn(async move {
        while let Some(_event) = handler.next().await {}
    });

    let page = browser
        .new_page(&file_url)
        .await
        .context("failed to create page for chrome framing")?;

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let screenshot = page
        .screenshot(
            chromiumoxide::page::ScreenshotParams::builder()
                .format(CaptureScreenshotFormat::Png)
                .full_page(true)
                .build(),
        )
        .await
        .context("failed to take chrome frame screenshot")?;

    let mut browser = browser;
    browser.close().await.ok();
    handle.await.ok();
    std::fs::remove_file(&tmp).ok();

    Ok(screenshot)
}
