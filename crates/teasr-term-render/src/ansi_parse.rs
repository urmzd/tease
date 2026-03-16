/// ANSI escape sequence parser that builds a cell grid from raw terminal output.
use anstyle_parse::{Params, Parser, Perform, Utf8Parser as CharAcc};

use crate::themes::Theme;

#[derive(Debug, Clone)]
pub struct Cell {
    pub ch: char,
    pub fg: CellColor,
    pub bg: CellColor,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub inverse: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CellColor {
    Default,
    Ansi(u8),
    Rgb(u8, u8, u8),
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: CellColor::Default,
            bg: CellColor::Default,
            bold: false,
            italic: false,
            underline: false,
            inverse: false,
        }
    }
}

impl Cell {
    pub fn resolve_fg(&self, theme: &Theme) -> &str {
        let (fg, bg) = if self.inverse {
            (&self.bg, &self.fg)
        } else {
            (&self.fg, &self.bg)
        };
        let _ = bg;
        match fg {
            CellColor::Default => theme.foreground,
            CellColor::Ansi(idx) => {
                let idx = if self.bold && *idx < 8 {
                    (*idx + 8) as usize
                } else {
                    *idx as usize
                };
                theme.ansi.get(idx).copied().unwrap_or(theme.foreground)
            }
            CellColor::Rgb(_, _, _) => theme.foreground, // handled by resolve_fg_rgb
        }
    }

    pub fn resolve_fg_rgb(&self) -> Option<(u8, u8, u8)> {
        let color = if self.inverse { &self.bg } else { &self.fg };
        match color {
            CellColor::Rgb(r, g, b) => Some((*r, *g, *b)),
            _ => None,
        }
    }

    pub fn resolve_bg(&self, theme: &Theme) -> &str {
        let color = if self.inverse { &self.fg } else { &self.bg };
        match color {
            CellColor::Default => theme.background,
            CellColor::Ansi(idx) => {
                theme.ansi.get(*idx as usize).copied().unwrap_or(theme.background)
            }
            CellColor::Rgb(_, _, _) => theme.background,
        }
    }

    pub fn resolve_bg_rgb(&self) -> Option<(u8, u8, u8)> {
        let color = if self.inverse { &self.fg } else { &self.bg };
        match color {
            CellColor::Rgb(r, g, b) => Some((*r, *g, *b)),
            _ => None,
        }
    }
}

/// A grid of cells representing terminal output.
#[derive(Debug, Clone)]
pub struct CellGrid {
    pub cols: usize,
    pub rows: Vec<Vec<Cell>>,
}

impl CellGrid {
    /// Returns (rows, cols) trimming trailing empty rows.
    pub fn dimensions(&self) -> (usize, usize) {
        let last_non_empty = self
            .rows
            .iter()
            .rposition(|row| row.iter().any(|c| c.ch != ' '))
            .map(|i| i + 1)
            .unwrap_or(1);
        (last_non_empty, self.cols)
    }
}

struct GridBuilder {
    cols: usize,
    max_rows: Option<usize>,
    rows: Vec<Vec<Cell>>,
    cursor_row: usize,
    cursor_col: usize,
    fg: CellColor,
    bg: CellColor,
    bold: bool,
    italic: bool,
    underline: bool,
    inverse: bool,
}

impl GridBuilder {
    fn new(cols: usize) -> Self {
        Self {
            cols,
            max_rows: None,
            rows: vec![vec![Cell::default(); cols]],
            cursor_row: 0,
            cursor_col: 0,
            fg: CellColor::Default,
            bg: CellColor::Default,
            bold: false,
            italic: false,
            underline: false,
            inverse: false,
        }
    }

    fn new_fixed(cols: usize, max_rows: usize) -> Self {
        let mut rows = Vec::with_capacity(max_rows);
        rows.push(vec![Cell::default(); cols]);
        Self {
            cols,
            max_rows: Some(max_rows),
            rows,
            cursor_row: 0,
            cursor_col: 0,
            fg: CellColor::Default,
            bg: CellColor::Default,
            bold: false,
            italic: false,
            underline: false,
            inverse: false,
        }
    }

    fn ensure_row(&mut self, row: usize) {
        while self.rows.len() <= row {
            self.rows.push(vec![Cell::default(); self.cols]);
        }
        // Scroll when exceeding max_rows
        if let Some(max) = self.max_rows {
            while self.rows.len() > max {
                self.rows.remove(0);
                self.cursor_row = self.cursor_row.saturating_sub(1);
            }
        }
    }

    fn put_char(&mut self, ch: char) {
        if self.cursor_col >= self.cols {
            self.cursor_col = 0;
            self.cursor_row += 1;
        }
        self.ensure_row(self.cursor_row);
        self.rows[self.cursor_row][self.cursor_col] = Cell {
            ch,
            fg: self.fg.clone(),
            bg: self.bg.clone(),
            bold: self.bold,
            italic: self.italic,
            underline: self.underline,
            inverse: self.inverse,
        };
        self.cursor_col += 1;
    }

    fn reset_attrs(&mut self) {
        self.fg = CellColor::Default;
        self.bg = CellColor::Default;
        self.bold = false;
        self.italic = false;
        self.underline = false;
        self.inverse = false;
    }

    fn handle_sgr(&mut self, params: &Params) {
        let mut iter = params.iter();
        let mut empty = true;
        while let Some(param) = iter.next() {
            empty = false;
            let code = param[0];
            match code {
                0 => self.reset_attrs(),
                1 => self.bold = true,
                3 => self.italic = true,
                4 => self.underline = true,
                7 => self.inverse = true,
                22 => self.bold = false,
                23 => self.italic = false,
                24 => self.underline = false,
                27 => self.inverse = false,
                30..=37 => self.fg = CellColor::Ansi((code - 30) as u8),
                38 => {
                    if let Some(mode) = iter.next() {
                        match mode[0] {
                            5 => {
                                if let Some(idx) = iter.next() {
                                    self.fg = ansi_256_to_color(idx[0] as u8);
                                }
                            }
                            2 => {
                                let r = iter.next().map(|p| p[0] as u8).unwrap_or(0);
                                let g = iter.next().map(|p| p[0] as u8).unwrap_or(0);
                                let b = iter.next().map(|p| p[0] as u8).unwrap_or(0);
                                self.fg = CellColor::Rgb(r, g, b);
                            }
                            _ => {}
                        }
                    }
                }
                39 => self.fg = CellColor::Default,
                40..=47 => self.bg = CellColor::Ansi((code - 40) as u8),
                48 => {
                    if let Some(mode) = iter.next() {
                        match mode[0] {
                            5 => {
                                if let Some(idx) = iter.next() {
                                    self.bg = ansi_256_to_color(idx[0] as u8);
                                }
                            }
                            2 => {
                                let r = iter.next().map(|p| p[0] as u8).unwrap_or(0);
                                let g = iter.next().map(|p| p[0] as u8).unwrap_or(0);
                                let b = iter.next().map(|p| p[0] as u8).unwrap_or(0);
                                self.bg = CellColor::Rgb(r, g, b);
                            }
                            _ => {}
                        }
                    }
                }
                49 => self.bg = CellColor::Default,
                90..=97 => self.fg = CellColor::Ansi((code - 90 + 8) as u8),
                100..=107 => self.bg = CellColor::Ansi((code - 100 + 8) as u8),
                _ => {}
            }
        }
        if empty {
            self.reset_attrs();
        }
    }
}

impl Perform for GridBuilder {
    fn print(&mut self, ch: char) {
        self.put_char(ch);
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.cursor_row += 1;
                self.cursor_col = 0;
                self.ensure_row(self.cursor_row);
            }
            b'\r' => {
                self.cursor_col = 0;
            }
            b'\t' => {
                let next_tab = (self.cursor_col / 8 + 1) * 8;
                let next_tab = next_tab.min(self.cols);
                while self.cursor_col < next_tab {
                    self.put_char(' ');
                }
            }
            0x08 => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                }
            }
            _ => {}
        }
    }

    fn csi_dispatch(&mut self, params: &Params, _intermediates: &[u8], _ignore: bool, action: u8) {
        match action {
            b'm' => self.handle_sgr(params),
            b'A' => {
                let n = first_param(params, 1) as usize;
                self.cursor_row = self.cursor_row.saturating_sub(n);
            }
            b'B' => {
                let n = first_param(params, 1) as usize;
                self.cursor_row += n;
                self.ensure_row(self.cursor_row);
            }
            b'C' => {
                let n = first_param(params, 1) as usize;
                self.cursor_col = (self.cursor_col + n).min(self.cols - 1);
            }
            b'D' => {
                let n = first_param(params, 1) as usize;
                self.cursor_col = self.cursor_col.saturating_sub(n);
            }
            b'G' => {
                let n = first_param(params, 1) as usize;
                self.cursor_col = (n.saturating_sub(1)).min(self.cols - 1);
            }
            b'H' | b'f' => {
                let row = first_param(params, 1) as usize;
                let col = second_param(params, 1) as usize;
                self.cursor_row = row.saturating_sub(1);
                self.cursor_col = col.saturating_sub(1).min(self.cols - 1);
                self.ensure_row(self.cursor_row);
            }
            b'J' => {
                let mode = first_param(params, 0);
                match mode {
                    0 => {
                        self.ensure_row(self.cursor_row);
                        for col in self.cursor_col..self.cols {
                            self.rows[self.cursor_row][col] = Cell::default();
                        }
                        for row in (self.cursor_row + 1)..self.rows.len() {
                            self.rows[row] = vec![Cell::default(); self.cols];
                        }
                    }
                    2 | 3 => {
                        for row in &mut self.rows {
                            *row = vec![Cell::default(); self.cols];
                        }
                    }
                    _ => {}
                }
            }
            b'K' => {
                let mode = first_param(params, 0);
                self.ensure_row(self.cursor_row);
                match mode {
                    0 => {
                        for col in self.cursor_col..self.cols {
                            self.rows[self.cursor_row][col] = Cell::default();
                        }
                    }
                    1 => {
                        for col in 0..=self.cursor_col.min(self.cols - 1) {
                            self.rows[self.cursor_row][col] = Cell::default();
                        }
                    }
                    2 => {
                        self.rows[self.cursor_row] = vec![Cell::default(); self.cols];
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {}
    fn hook(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _action: u8) {}
    fn unhook(&mut self) {}
    fn put(&mut self, _byte: u8) {}
    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {}
}

fn first_param(params: &Params, default: u16) -> u16 {
    params.iter().next().map(|p| p[0]).unwrap_or(default)
}

fn second_param(params: &Params, default: u16) -> u16 {
    params.iter().nth(1).map(|p| p[0]).unwrap_or(default)
}

fn ansi_256_to_color(idx: u8) -> CellColor {
    match idx {
        0..=15 => CellColor::Ansi(idx),
        16..=231 => {
            let idx = idx - 16;
            let r = (idx / 36) * 51;
            let g = ((idx % 36) / 6) * 51;
            let b = (idx % 6) * 51;
            CellColor::Rgb(r, g, b)
        }
        232..=255 => {
            let gray = 8 + (idx - 232) * 10;
            CellColor::Rgb(gray, gray, gray)
        }
    }
}

/// A snapshotable terminal emulator for incremental byte feeding.
pub struct TerminalEmulator {
    parser: Parser<CharAcc>,
    builder: GridBuilder,
}

impl TerminalEmulator {
    /// Create a fixed-size terminal emulator.
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            parser: Parser::<CharAcc>::new(),
            builder: GridBuilder::new_fixed(cols, rows),
        }
    }

    /// Feed raw bytes into the emulator incrementally.
    pub fn feed(&mut self, data: &[u8]) {
        for &byte in data {
            self.parser.advance(&mut self.builder, byte);
        }
    }

    /// Snapshot the current visible grid.
    pub fn snapshot(&self) -> CellGrid {
        CellGrid {
            cols: self.builder.cols,
            rows: self.builder.rows.clone(),
        }
    }
}

/// Parse raw ANSI bytes into a cell grid.
pub fn parse(input: &[u8], cols: usize) -> CellGrid {
    let mut parser = Parser::<CharAcc>::new();
    let mut builder = GridBuilder::new(cols);

    for &byte in input {
        parser.advance(&mut builder, byte);
    }

    CellGrid {
        cols: builder.cols,
        rows: builder.rows,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text() {
        let grid = parse(b"Hello, world!", 80);
        let (rows, cols) = grid.dimensions();
        assert_eq!(rows, 1);
        assert_eq!(cols, 80);
        let text: String = grid.rows[0].iter().take(13).map(|c| c.ch).collect();
        assert_eq!(text, "Hello, world!");
    }

    #[test]
    fn newlines() {
        let grid = parse(b"line1\nline2\nline3", 80);
        let (rows, _) = grid.dimensions();
        assert_eq!(rows, 3);
    }

    #[test]
    fn sgr_bold_color() {
        let input = b"\x1b[1;31mred bold\x1b[0m normal";
        let grid = parse(input, 80);
        assert!(grid.rows[0][0].bold);
        assert_eq!(grid.rows[0][0].fg, CellColor::Ansi(1));
        assert!(!grid.rows[0][9].bold);
    }

    #[test]
    fn line_wrapping() {
        let grid = parse(b"abcde", 3);
        assert_eq!(grid.rows[0][0].ch, 'a');
        assert_eq!(grid.rows[0][2].ch, 'c');
        assert_eq!(grid.rows[1][0].ch, 'd');
        assert_eq!(grid.rows[1][1].ch, 'e');
    }

    #[test]
    fn color_256() {
        let input = b"\x1b[38;5;196mred\x1b[0m";
        let grid = parse(input, 80);
        assert!(matches!(grid.rows[0][0].fg, CellColor::Rgb(255, 0, 0)));
    }

    #[test]
    fn color_rgb() {
        let input = b"\x1b[38;2;100;200;50mcolor\x1b[0m";
        let grid = parse(input, 80);
        assert_eq!(grid.rows[0][0].fg, CellColor::Rgb(100, 200, 50));
    }

    #[test]
    fn emulator_incremental_feed() {
        let mut emu = TerminalEmulator::new(80, 24);
        emu.feed(b"Hello");
        let grid = emu.snapshot();
        let text: String = grid.rows[0].iter().take(5).map(|c| c.ch).collect();
        assert_eq!(text, "Hello");

        emu.feed(b", world!");
        let grid = emu.snapshot();
        let text: String = grid.rows[0].iter().take(13).map(|c| c.ch).collect();
        assert_eq!(text, "Hello, world!");
    }

    #[test]
    fn emulator_scrolling() {
        let mut emu = TerminalEmulator::new(80, 3);
        emu.feed(b"line1\nline2\nline3\nline4\nline5");
        let grid = emu.snapshot();
        // Should have scrolled: only 3 rows visible
        assert_eq!(grid.rows.len(), 3);
        let first: String = grid.rows[0].iter().take(5).map(|c| c.ch).collect();
        let last: String = grid.rows[2].iter().take(5).map(|c| c.ch).collect();
        assert_eq!(first, "line3");
        assert_eq!(last, "line5");
    }

    #[test]
    fn emulator_snapshot_is_independent() {
        let mut emu = TerminalEmulator::new(80, 24);
        emu.feed(b"before");
        let snap1 = emu.snapshot();
        emu.feed(b"\nafter");
        let snap2 = emu.snapshot();
        // snap1 should not be affected by subsequent feed
        assert_eq!(snap1.rows.len(), 1);
        assert!(snap2.rows.len() >= 2);
    }
}
