pub mod ansi_parse;
pub mod rasterize;
pub mod svg;
pub mod themes;

use anyhow::Result;

pub use ansi_parse::{CellGrid, TerminalEmulator};

/// Render raw ANSI terminal output to a PNG image.
///
/// # Arguments
/// * `input` - Raw bytes from terminal (including ANSI escape sequences)
/// * `cols` - Terminal width in columns
/// * `theme_name` - Theme name ("dracula", "monokai")
/// * `title` - Optional title shown in the terminal chrome
pub fn render_to_png(
    input: &[u8],
    cols: usize,
    theme_name: &str,
    title: Option<&str>,
) -> Result<Vec<u8>> {
    let theme = themes::get_theme(theme_name);
    let grid = ansi_parse::parse(input, cols);
    let svg_str = svg::render(
        &grid,
        theme,
        &svg::SvgOptions {
            title: title.map(|s| s.to_string()),
        },
    );
    rasterize::svg_to_png(&svg_str)
}

/// Render a CellGrid directly to PNG bytes.
pub fn render_grid_to_png(
    grid: &CellGrid,
    theme_name: &str,
    title: Option<&str>,
) -> Result<Vec<u8>> {
    let theme = themes::get_theme(theme_name);
    let svg_str = svg::render(
        grid,
        theme,
        &svg::SvgOptions {
            title: title.map(|s| s.to_string()),
        },
    );
    rasterize::svg_to_png(&svg_str)
}
