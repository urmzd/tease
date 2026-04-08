use anyhow::{Context, Result};
use image::GenericImageView;

use crate::ansi_parse::{Cell, CellColor, CellGrid, TerminalEmulator};
use crate::RenderOptions;

/// Vertical alignment for splash content.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum VAlign {
    #[default]
    Top,
    Center,
    Bottom,
}

/// Render a text splash screen to PNG bytes.
///
/// The text is placed in a CellGrid of the given dimensions, optionally centered.
pub fn render_text_splash(
    text: &str,
    cols: usize,
    rows: usize,
    center: bool,
    v_align: VAlign,
    opts: &RenderOptions,
) -> Result<Vec<u8>> {
    let grid = text_to_grid(text, cols, rows, center, v_align);
    crate::render_grid_to_png(&grid, opts)
}

/// Render an ANSI file (.ans/.txt) splash screen to PNG bytes.
///
/// The file content is fed through a terminal emulator to parse ANSI escape codes.
pub fn render_ansi_splash(
    content: &[u8],
    cols: usize,
    rows: usize,
    _center: bool,
    v_align: VAlign,
    opts: &RenderOptions,
) -> Result<Vec<u8>> {
    let mut emu = TerminalEmulator::new(cols, rows);
    emu.feed(content);
    let grid = emu.snapshot();

    if v_align == VAlign::Top {
        return crate::render_grid_to_png(&grid, opts);
    }

    // Find used rows for vertical positioning
    let used_rows = grid
        .rows
        .iter()
        .rposition(|row| row.iter().any(|c| c.ch != ' '))
        .map_or(0, |r| r + 1);
    let v_offset = if used_rows < rows {
        match v_align {
            VAlign::Center => (rows - used_rows) / 2,
            VAlign::Bottom => rows - used_rows,
            VAlign::Top => unreachable!(),
        }
    } else {
        0
    };

    if v_offset == 0 {
        return crate::render_grid_to_png(&grid, opts);
    }

    let mut padded = CellGrid {
        cols,
        rows: Vec::with_capacity(rows),
    };
    let empty_row = vec![Cell::default(); cols];
    for _ in 0..v_offset {
        padded.rows.push(empty_row.clone());
    }
    for row in grid.rows.into_iter().take(rows - v_offset) {
        padded.rows.push(row);
    }
    while padded.rows.len() < rows {
        padded.rows.push(empty_row.clone());
    }
    crate::render_grid_to_png(&padded, opts)
}

/// Render an image splash screen to PNG bytes.
///
/// The image is composited onto an empty terminal frame.
pub fn render_image_splash(
    image_data: &[u8],
    cols: usize,
    rows: usize,
    center: bool,
    v_align: VAlign,
    opts: &RenderOptions,
) -> Result<Vec<u8>> {
    // First render an empty terminal frame
    let empty_grid = CellGrid {
        cols,
        rows: vec![vec![Cell::default(); cols]; rows],
    };
    let bg_png = crate::render_grid_to_png(&empty_grid, opts)?;

    // Load both images
    let bg_img = image::load_from_memory(&bg_png).context("failed to decode background PNG")?;
    let overlay_img =
        image::load_from_memory(image_data).context("failed to decode overlay image")?;

    let mut canvas = bg_img.to_rgba8();
    let (canvas_w, canvas_h) = canvas.dimensions();

    // Scale the overlay to fit within the content area (with padding)
    let padding = 32u32; // 16px padding on each side
    let chrome_height = 40u32;
    let content_w = canvas_w.saturating_sub(padding);
    let content_h = canvas_h.saturating_sub(chrome_height + padding);

    let (ow, oh) = overlay_img.dimensions();
    let scale = f64::min(
        content_w as f64 / ow as f64,
        content_h as f64 / oh as f64,
    )
    .min(1.0); // Don't upscale

    let scaled_w = (ow as f64 * scale) as u32;
    let scaled_h = (oh as f64 * scale) as u32;

    let resized = image::imageops::resize(
        &overlay_img.to_rgba8(),
        scaled_w,
        scaled_h,
        image::imageops::FilterType::Lanczos3,
    );

    let x = if center {
        ((canvas_w - scaled_w) / 2) as i64
    } else {
        padding as i64 / 2
    };
    let y = match v_align {
        VAlign::Top => (chrome_height + padding / 2) as i64,
        VAlign::Center => chrome_height as i64 + (content_h as i64 - scaled_h as i64) / 2,
        VAlign::Bottom => chrome_height as i64 + content_h as i64 - scaled_h as i64,
    };

    image::imageops::overlay(&mut canvas, &resized, x, y);

    let mut buf = std::io::Cursor::new(Vec::new());
    canvas
        .write_to(&mut buf, image::ImageFormat::Png)
        .context("failed to encode splash PNG")?;
    Ok(buf.into_inner())
}

/// Convert plain text into a CellGrid, optionally centering.
fn text_to_grid(text: &str, cols: usize, rows: usize, center: bool, v_align: VAlign) -> CellGrid {
    let lines: Vec<&str> = text.lines().collect();
    let num_lines = lines.len();

    let v_offset = if num_lines < rows {
        match v_align {
            VAlign::Top => 0,
            VAlign::Center => (rows - num_lines) / 2,
            VAlign::Bottom => rows - num_lines,
        }
    } else {
        0
    };

    let mut grid_rows = Vec::with_capacity(rows);

    for r in 0..rows {
        let line_idx = r.checked_sub(v_offset);
        let line = line_idx.and_then(|i| lines.get(i)).copied().unwrap_or("");

        let chars: Vec<char> = line.chars().collect();
        let h_offset = if center && chars.len() < cols {
            (cols - chars.len()) / 2
        } else {
            0
        };

        let mut row = vec![Cell::default(); cols];
        for (i, ch) in chars.iter().enumerate() {
            let col = h_offset + i;
            if col < cols {
                row[col] = Cell {
                    ch: *ch,
                    fg: CellColor::Default,
                    bg: CellColor::Default,
                    bold: false,
                    italic: false,
                    underline: false,
                    inverse: false,
                };
            }
        }
        grid_rows.push(row);
    }

    CellGrid {
        cols,
        rows: grid_rows,
    }
}
