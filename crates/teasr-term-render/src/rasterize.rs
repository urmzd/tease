/// Rasterize SVG to PNG using resvg.
use anyhow::{Context, Result};
use resvg::tiny_skia;
use resvg::usvg;
use std::path::Path;
use std::sync::{Arc, LazyLock, Mutex};

static FONTDB: LazyLock<Mutex<Arc<usvg::fontdb::Database>>> = LazyLock::new(|| {
    let mut db = usvg::fontdb::Database::new();
    db.load_system_fonts();
    Mutex::new(Arc::new(db))
});

/// Check if a font family is available (in system fonts or loaded extras).
pub fn check_font_available(family: &str) -> bool {
    let arc = FONTDB.lock().unwrap();
    let result = arc.faces().any(|face| {
        face.families
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case(family))
    });
    result
}

/// Load an extra font file (.ttf/.otf) into the global font database.
pub fn load_extra_font(path: &Path) -> Result<()> {
    let data = std::fs::read(path)
        .with_context(|| format!("failed to read font file: {}", path.display()))?;
    let mut arc = FONTDB.lock().unwrap();
    let db = Arc::make_mut(&mut arc);
    db.load_font_data(data);
    Ok(())
}

/// Convert an SVG string to PNG bytes.
pub fn svg_to_png(svg: &str, font_family: Option<&str>) -> Result<Vec<u8>> {
    let fontdb = FONTDB.lock().unwrap().clone();
    let opts = usvg::Options {
        font_family: font_family.unwrap_or("monospace").to_string(),
        fontdb,
        ..Default::default()
    };

    let tree = usvg::Tree::from_str(svg, &opts).context("failed to parse SVG")?;

    let size = tree.size();
    let width = size.width().ceil() as u32;
    let height = size.height().ceil() as u32;

    let mut pixmap = tiny_skia::Pixmap::new(width, height).context("failed to create pixmap")?;

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
        let png = svg_to_png(svg, None).unwrap();
        // PNG magic bytes
        assert_eq!(&png[..4], &[0x89, 0x50, 0x4e, 0x47]);
    }
}
