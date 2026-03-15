/// Rasterize SVG to PNG using resvg.
use anyhow::{Context, Result};
use resvg::tiny_skia;
use resvg::usvg;
use std::sync::{Arc, LazyLock};

// Embed JetBrains Mono font for consistent rendering
const JETBRAINS_MONO_REGULAR: &[u8] =
    include_bytes!("../assets/JetBrainsMono-Regular.ttf");
const JETBRAINS_MONO_BOLD: &[u8] =
    include_bytes!("../assets/JetBrainsMono-Bold.ttf");
const JETBRAINS_MONO_ITALIC: &[u8] =
    include_bytes!("../assets/JetBrainsMono-Italic.ttf");
const JETBRAINS_MONO_BOLD_ITALIC: &[u8] =
    include_bytes!("../assets/JetBrainsMono-BoldItalic.ttf");

static FONTDB: LazyLock<Arc<usvg::fontdb::Database>> = LazyLock::new(|| {
    let mut db = usvg::fontdb::Database::new();
    db.load_font_data(JETBRAINS_MONO_REGULAR.to_vec());
    db.load_font_data(JETBRAINS_MONO_BOLD.to_vec());
    db.load_font_data(JETBRAINS_MONO_ITALIC.to_vec());
    db.load_font_data(JETBRAINS_MONO_BOLD_ITALIC.to_vec());
    Arc::new(db)
});

/// Convert an SVG string to PNG bytes.
pub fn svg_to_png(svg: &str) -> Result<Vec<u8>> {
    let opts = usvg::Options {
        font_family: "JetBrains Mono".to_string(),
        fontdb: FONTDB.clone(),
        ..Default::default()
    };
    let tree = usvg::Tree::from_str(svg, &opts)
        .context("failed to parse SVG")?;

    let size = tree.size();
    let width = size.width().ceil() as u32;
    let height = size.height().ceil() as u32;

    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .context("failed to create pixmap")?;

    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    pixmap.encode_png().context("failed to encode PNG")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_svg_to_png() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
            <rect width="100" height="100" fill="red"/>
        </svg>"#;
        let png = svg_to_png(svg).unwrap();
        // PNG magic bytes
        assert_eq!(&png[..4], &[0x89, 0x50, 0x4e, 0x47]);
    }
}
