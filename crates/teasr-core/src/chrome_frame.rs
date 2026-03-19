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

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
<style>
* {{ margin: 0; padding: 0; box-sizing: border-box; }}
body {{
  background: transparent;
  display: flex;
  justify-content: center;
  align-items: center;
  width: {vp_w}px;
  height: {vp_h}px;
}}
.window {{
  width: {win_w}px;
  border-radius: 10px;
  overflow: hidden;
  box-shadow: 0 20px 60px rgba(0,0,0,0.5), 0 0 0 1px rgba(255,255,255,0.05);
}}
.chrome {{
  height: 40px;
  background: {chrome_bg};
  display: flex;
  align-items: center;
  padding: 0 16px;
  position: relative;
}}
.buttons {{
  display: flex;
  gap: 8px;
}}
.btn {{
  width: 12px;
  height: 12px;
  border-radius: 50%;
}}
.btn-close {{ background: {btn_close}; }}
.btn-min {{ background: {btn_min}; }}
.btn-max {{ background: {btn_max}; }}
.title {{
  position: absolute;
  left: 0;
  right: 0;
  text-align: center;
  font-family: 'JetBrains Mono', 'SF Mono', monospace;
  font-size: 13px;
  color: {fg};
  pointer-events: none;
}}
.content {{
  background: {bg};
  padding: 16px;
}}
.content img {{
  display: block;
  width: {img_w}px;
  height: {img_h}px;
}}
</style>
</head>
<body>
<div class="window">
  <div class="chrome">
    <div class="buttons">
      <div class="btn btn-close"></div>
      <div class="btn btn-min"></div>
      <div class="btn btn-max"></div>
    </div>
    <div class="title">{title}</div>
  </div>
  <div class="content">
    <img src="data:image/png;base64,{b64}">
  </div>
</div>
</body>
</html>"#,
        vp_w = render_w + 32 + 80,  // content padding (16*2) + body padding (40*2)
        vp_h = render_h + 40 + 32 + 80,  // chrome + content padding + body padding
        win_w = render_w + 32,  // content padding
        chrome_bg = chrome_bg,
        btn_close = btn_close,
        btn_min = btn_min,
        btn_max = btn_max,
        fg = fg,
        bg = bg,
        img_w = render_w,
        img_h = render_h,
        title = title_text,
        b64 = b64,
    );

    let html_b64 = base64::engine::general_purpose::STANDARD.encode(html.as_bytes());
    let data_url = format!("data:text/html;base64,{html_b64}");

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
        .new_page(&data_url)
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

    Ok(screenshot)
}
