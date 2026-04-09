use anyhow::{Context, Result};
use base64::Engine;
use tracing::debug;

use crate::browser::{self, LaunchOptions};
use crate::capture::chrome::{install_idle_tracker, page_has_activity};
use crate::capture::wait_for_idle;

/// Render a PNG image inside macOS-style window chrome using a headless browser.
///
/// Visual constants match `teasr-term-render/src/svg.rs` and `themes.rs`:
/// chrome height 40px, corner radius 10px, padding 16px, Dracula theme by default.
pub async fn render_with_chrome_frame(
    png_data: &[u8],
    title: Option<&str>,
    theme: &str,
) -> Result<Vec<u8>> {
    let img = image::load_from_memory(png_data).context("failed to decode PNG for framing")?;
    let render_w = img.width();
    let render_h = img.height();

    let (bg, chrome_bg, fg, btn_close, btn_min, btn_max) = match theme {
        "monokai" => ("#272822", "#1e1f1c", "#f8f8f2", "#f92672", "#f4bf75", "#a6e22e"),
        _ => ("#282a36", "#1e1f29", "#f8f8f2", "#ff5555", "#f1fa8c", "#50fa7b"),
    };

    let title_text = title.unwrap_or("Screen Capture");
    let b64 = base64::engine::general_purpose::STANDARD.encode(png_data);

    let html = include_str!("chrome_frame.html")
        .replace("__VP_W__", &(render_w + 32 + 80).to_string())
        .replace("__VP_H__", &(render_h + 40 + 32 + 80).to_string())
        .replace("__WIN_W__", &(render_w + 32).to_string())
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

    let mut browser = browser::launch(LaunchOptions::new(vp_w, vp_h)).await?;
    let page = browser.new_page(&file_url).await?;

    install_idle_tracker(&*page).await;
    wait_for_idle(5000, 500, 50, || page_has_activity(&*page)).await;

    let screenshot = page.screenshot(true).await?;

    browser.close().await;
    std::fs::remove_file(&tmp).ok();

    Ok(screenshot)
}
