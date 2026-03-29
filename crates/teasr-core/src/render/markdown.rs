use anyhow::{Context, Result};
use comrak::{markdown_to_html, Options};
use std::path::Path;
use tempfile::NamedTempFile;

use crate::types::{MarkdownFlavor, MarkdownTheme};

const TEMPLATE: &str = include_str!("../markdown_template.html");

struct Palette {
    fg: &'static str,
    bg: &'static str,
    heading: &'static str,
    link: &'static str,
    muted: &'static str,
    border: &'static str,
    code_bg: &'static str,
    stripe: &'static str,
}

const LIGHT: Palette = Palette {
    fg: "#1f2328",
    bg: "#ffffff",
    heading: "#1f2328",
    link: "#0969da",
    muted: "#656d76",
    border: "#d0d7de",
    code_bg: "#f6f8fa",
    stripe: "#f6f8fa",
};

const DARK: Palette = Palette {
    fg: "#e6edf3",
    bg: "#0d1117",
    heading: "#e6edf3",
    link: "#58a6ff",
    muted: "#8b949e",
    border: "#30363d",
    code_bg: "#161b22",
    stripe: "#161b22",
};

fn palette_for(theme: &MarkdownTheme) -> &'static Palette {
    match theme {
        MarkdownTheme::Light => &LIGHT,
        MarkdownTheme::Dark => &DARK,
    }
}

fn comrak_options(flavor: &MarkdownFlavor) -> Options<'_> {
    let mut opts = Options::default();
    match flavor {
        MarkdownFlavor::Commonmark => {
            // Strict CommonMark — no extensions.
        }
        MarkdownFlavor::Github | MarkdownFlavor::Custom => {
            opts.extension.strikethrough = true;
            opts.extension.table = true;
            opts.extension.autolink = true;
            opts.extension.tasklist = true;
            opts.extension.footnotes = true;
            opts.extension.header_ids = Some(String::new());
        }
    }
    opts.render.unsafe_ = true; // Allow raw HTML in markdown
    opts
}

/// Render a markdown file to a temporary HTML file ready for Chrome capture.
///
/// Returns the `NamedTempFile` — the caller must keep it alive until capture is done.
pub fn render_to_html(
    md_path: &Path,
    flavor: &MarkdownFlavor,
    theme: &MarkdownTheme,
    stylesheet: Option<&Path>,
    template: Option<&Path>,
    viewport_width: u32,
) -> Result<NamedTempFile> {
    let md_source = std::fs::read_to_string(md_path)
        .with_context(|| format!("failed to read markdown file: {}", md_path.display()))?;

    let opts = comrak_options(flavor);
    let html_body = markdown_to_html(&md_source, &opts);

    let full_html = if let Some(tpl_path) = template {
        // User-provided template — just substitute {{content}}.
        let tpl = std::fs::read_to_string(tpl_path)
            .with_context(|| format!("failed to read template: {}", tpl_path.display()))?;
        tpl.replace("{{content}}", &html_body)
    } else {
        // Use the bundled template with palette substitution.
        let palette = palette_for(theme);
        let mut html = TEMPLATE.to_string();
        html = html.replace("__VP_W__", &viewport_width.to_string());
        html = html.replace("__FG__", palette.fg);
        html = html.replace("__BG__", palette.bg);
        html = html.replace("__HEADING__", palette.heading);
        html = html.replace("__LINK__", palette.link);
        html = html.replace("__MUTED__", palette.muted);
        html = html.replace("__BORDER__", palette.border);
        html = html.replace("__CODE_BG__", palette.code_bg);
        html = html.replace("__STRIPE__", palette.stripe);

        // Syntax highlighting CSS from comrak/syntect (already inline in code spans).
        html = html.replace("__SYNTAX_CSS__", "");

        // Custom stylesheet injection.
        let custom_css = if let Some(css_path) = stylesheet {
            let css = std::fs::read_to_string(css_path)
                .with_context(|| format!("failed to read stylesheet: {}", css_path.display()))?;
            format!("<style>\n{css}\n</style>")
        } else {
            String::new()
        };
        html = html.replace("__CUSTOM_CSS__", &custom_css);

        html = html.replace("__CONTENT__", &html_body);
        html
    };

    let mut tmp = NamedTempFile::with_suffix(".html")
        .context("failed to create temp file for markdown render")?;
    std::io::Write::write_all(&mut tmp, full_html.as_bytes())
        .context("failed to write rendered markdown HTML")?;
    Ok(tmp)
}
