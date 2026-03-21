pub mod ansi_parse;
pub mod rasterize;
pub mod splash;
pub mod svg;
pub mod themes;

use anyhow::Result;

pub use ansi_parse::{CellGrid, TerminalEmulator};
pub use rasterize::{check_font_available, load_extra_font};

/// Options for rendering terminal output to PNG.
pub struct RenderOptions<'a> {
    pub theme_name: &'a str,
    pub title: Option<&'a str>,
    pub font_family: Option<&'a str>,
    pub font_size: Option<f64>,
}

/// Render raw ANSI terminal output to a PNG image.
pub fn render_to_png(input: &[u8], cols: usize, opts: &RenderOptions) -> Result<Vec<u8>> {
    let theme = themes::get_theme(opts.theme_name);
    let grid = ansi_parse::parse(input, cols);
    let svg_str = svg::render(
        &grid,
        theme,
        &svg::SvgOptions {
            title: opts.title.map(|s| s.to_string()),
            font_family: opts.font_family.map(|s| s.to_string()),
            font_size: opts.font_size,
        },
    );
    rasterize::svg_to_png(&svg_str, opts.font_family)
}

/// Render a CellGrid directly to PNG bytes.
pub fn render_grid_to_png(grid: &CellGrid, opts: &RenderOptions) -> Result<Vec<u8>> {
    let theme = themes::get_theme(opts.theme_name);
    let svg_str = svg::render(
        grid,
        theme,
        &svg::SvgOptions {
            title: opts.title.map(|s| s.to_string()),
            font_family: opts.font_family.map(|s| s.to_string()),
            font_size: opts.font_size,
        },
    );
    rasterize::svg_to_png(&svg_str, opts.font_family)
}
