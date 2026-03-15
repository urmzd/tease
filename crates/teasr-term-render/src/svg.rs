/// SVG builder that renders a cell grid into an SVG string with terminal chrome.
use crate::ansi_parse::{Cell, CellGrid};
use crate::themes::Theme;

const CELL_WIDTH: f64 = 9.6;
const CELL_HEIGHT: f64 = 20.0;
const PADDING: f64 = 16.0;
const CHROME_HEIGHT: f64 = 40.0;
const BUTTON_RADIUS: f64 = 6.0;
const BUTTON_Y: f64 = 20.0;
const BUTTON_START_X: f64 = 20.0;
const BUTTON_GAP: f64 = 20.0;
const CORNER_RADIUS: f64 = 10.0;

// JetBrains Mono is embedded at rasterization time; SVG just references it by name.
const FONT_FAMILY: &str = "JetBrains Mono, monospace";
const FONT_SIZE: f64 = 14.0;

#[derive(Default)]
pub struct SvgOptions {
    pub title: Option<String>,
}

/// Render a cell grid to an SVG string.
pub fn render(grid: &CellGrid, theme: &Theme, opts: &SvgOptions) -> String {
    let (num_rows, num_cols) = grid.dimensions();
    let content_width = num_cols as f64 * CELL_WIDTH;
    let content_height = num_rows as f64 * CELL_HEIGHT;
    let total_width = content_width + PADDING * 2.0;
    let total_height = content_height + PADDING * 2.0 + CHROME_HEIGHT;

    let mut svg = String::with_capacity(num_rows * num_cols * 80);

    // SVG header
    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{total_width}" height="{total_height}" viewBox="0 0 {total_width} {total_height}">"#,
    ));

    // Background with rounded corners
    svg.push_str(&format!(
        r#"<rect width="{total_width}" height="{total_height}" rx="{CORNER_RADIUS}" fill="{bg}"/>"#,
        bg = theme.background,
    ));

    // Title bar chrome
    svg.push_str(&format!(
        r#"<rect width="{total_width}" height="{CHROME_HEIGHT}" rx="{CORNER_RADIUS}" fill="{chrome_bg}"/>"#,
        chrome_bg = theme.chrome_bg,
    ));
    // Bottom corners of chrome (fill the rounded bottom)
    svg.push_str(&format!(
        r#"<rect y="{y}" width="{total_width}" height="{CORNER_RADIUS}" fill="{chrome_bg}"/>"#,
        y = CHROME_HEIGHT - CORNER_RADIUS,
        chrome_bg = theme.chrome_bg,
    ));

    // Traffic light buttons
    for (i, color) in theme.chrome_buttons.iter().enumerate() {
        let cx = BUTTON_START_X + i as f64 * BUTTON_GAP;
        svg.push_str(&format!(
            r#"<circle cx="{cx}" cy="{BUTTON_Y}" r="{BUTTON_RADIUS}" fill="{color}"/>"#,
        ));
    }

    // Title text (if provided)
    if let Some(title) = &opts.title {
        svg.push_str(&format!(
            r#"<text x="{x}" y="{y}" font-family="{FONT_FAMILY}" font-size="13" fill="{fg}" text-anchor="middle">{title}</text>"#,
            x = total_width / 2.0,
            y = BUTTON_Y + 4.0,
            fg = theme.foreground,
            title = escape_xml(title),
        ));
    }

    // Content area - render cells
    let base_y = CHROME_HEIGHT + PADDING;
    let base_x = PADDING;

    for (row_idx, row) in grid.rows.iter().enumerate().take(num_rows) {
        let y = base_y + row_idx as f64 * CELL_HEIGHT;

        // Group background spans for efficiency
        let bg_spans = collect_bg_spans(row, num_cols, theme);
        for span in &bg_spans {
            let x = base_x + span.start as f64 * CELL_WIDTH;
            let w = (span.end - span.start) as f64 * CELL_WIDTH;
            svg.push_str(&format!(
                r#"<rect x="{x}" y="{y}" width="{w}" height="{CELL_HEIGHT}" fill="{color}"/>"#,
                color = span.color,
            ));
        }

        // Group text spans by style
        let text_spans = collect_text_spans(row, num_cols, theme);
        for span in &text_spans {
            let x = base_x + span.start as f64 * CELL_WIDTH;
            let text_y = y + CELL_HEIGHT - 5.0;

            let mut style = format!("fill:{}", span.color);
            if span.bold {
                style.push_str(";font-weight:bold");
            }
            if span.italic {
                style.push_str(";font-style:italic");
            }

            svg.push_str(&format!(
                r#"<text x="{x}" y="{text_y}" font-family="{FONT_FAMILY}" font-size="{FONT_SIZE}" style="{style}" xml:space="preserve">{text}</text>"#,
                text = escape_xml(&span.text),
            ));

            if span.underline {
                let uy = text_y + 2.0;
                let w = span.text.chars().count() as f64 * CELL_WIDTH;
                svg.push_str(&format!(
                    r#"<line x1="{x}" y1="{uy}" x2="{x2}" y2="{uy}" stroke="{color}" stroke-width="1"/>"#,
                    x2 = x + w,
                    color = span.color,
                ));
            }
        }
    }

    svg.push_str("</svg>");
    svg
}

struct BgSpan {
    start: usize,
    end: usize,
    color: String,
}

struct TextSpan {
    start: usize,
    text: String,
    color: String,
    bold: bool,
    italic: bool,
    underline: bool,
}

fn resolve_fg_color(cell: &Cell, theme: &Theme) -> String {
    if let Some((r, g, b)) = cell.resolve_fg_rgb() {
        format!("rgb({r},{g},{b})")
    } else {
        cell.resolve_fg(theme).to_string()
    }
}

fn resolve_bg_color(cell: &Cell, theme: &Theme) -> String {
    if let Some((r, g, b)) = cell.resolve_bg_rgb() {
        format!("rgb({r},{g},{b})")
    } else {
        cell.resolve_bg(theme).to_string()
    }
}

fn collect_bg_spans(row: &[Cell], num_cols: usize, theme: &Theme) -> Vec<BgSpan> {
    let mut spans = Vec::new();
    let mut i = 0;
    while i < num_cols && i < row.len() {
        let color = resolve_bg_color(&row[i], theme);
        if color == theme.background {
            i += 1;
            continue;
        }
        let start = i;
        while i < num_cols && i < row.len() && resolve_bg_color(&row[i], theme) == color {
            i += 1;
        }
        spans.push(BgSpan {
            start,
            end: i,
            color,
        });
    }
    spans
}

fn collect_text_spans(row: &[Cell], num_cols: usize, theme: &Theme) -> Vec<TextSpan> {
    let mut spans = Vec::new();
    let mut i = 0;
    while i < num_cols && i < row.len() {
        // Skip spaces with default styling
        if row[i].ch == ' '
            && !row[i].bold
            && !row[i].italic
            && !row[i].underline
        {
            i += 1;
            continue;
        }

        let color = resolve_fg_color(&row[i], theme);
        let bold = row[i].bold;
        let italic = row[i].italic;
        let underline = row[i].underline;
        let start = i;
        let mut text = String::new();

        while i < num_cols
            && i < row.len()
            && resolve_fg_color(&row[i], theme) == color
            && row[i].bold == bold
            && row[i].italic == italic
            && row[i].underline == underline
        {
            text.push(row[i].ch);
            i += 1;
        }

        // Trim trailing spaces from span
        let trimmed = text.trim_end();
        if !trimmed.is_empty() {
            spans.push(TextSpan {
                start,
                text: trimmed.to_string(),
                color,
                bold,
                italic,
                underline,
            });
        }
    }
    spans
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ansi_parse::parse;
    use crate::themes::DRACULA;

    #[test]
    fn renders_valid_svg() {
        let grid = parse(b"Hello", 80);
        let svg = render(&grid, &DRACULA, &SvgOptions::default());
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
        assert!(svg.contains("Hello"));
    }

    #[test]
    fn renders_with_title() {
        let grid = parse(b"test", 80);
        let svg = render(
            &grid,
            &DRACULA,
            &SvgOptions {
                title: Some("My Terminal".into()),
            },
        );
        assert!(svg.contains("My Terminal"));
    }

    #[test]
    fn escapes_special_chars() {
        let grid = parse(b"<script>&\"test\"</script>", 80);
        let svg = render(&grid, &DRACULA, &SvgOptions::default());
        assert!(!svg.contains("<script>"));
        assert!(svg.contains("&lt;script&gt;"));
    }
}
