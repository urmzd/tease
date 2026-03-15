# teasr-term-render

ANSI terminal output renderer. Converts raw terminal bytes (including ANSI escape sequences) into styled PNG images with terminal chrome — title bar, traffic light buttons, themed colors, and JetBrains Mono font.

Zero external tool dependencies: font is embedded, SVG rasterization uses resvg.

## Pipeline

```
raw bytes (ANSI)
  → anstyle-parse            (escape sequence parsing)
  → cell grid                (character + style per cell)
  → SVG string               (terminal chrome + styled text)
  → resvg + tiny-skia        (rasterization)
  → PNG bytes
```

## Usage

```toml
[dependencies]
teasr-term-render = "0.2"
```

```rust
use teasr_term_render::render_to_png;

let ansi_output: Vec<u8> = run_command_and_capture("cargo build 2>&1");

let png_bytes = render_to_png(
    &ansi_output,
    100,            // terminal width in columns
    "dracula",      // theme name
    Some("cargo build"),  // title shown in the title bar (None to omit)
)?;

std::fs::write("output.png", &png_bytes)?;
```

## API

```rust
pub fn render_to_png(
    input: &[u8],
    cols: usize,
    theme_name: &str,
    title: Option<&str>,
) -> anyhow::Result<Vec<u8>>
```

| Parameter | Description |
|-----------|-------------|
| `input` | Raw bytes from terminal output, including ANSI escape sequences |
| `cols` | Terminal width in columns (affects line wrapping) |
| `theme_name` | `"dracula"` or `"monokai"` (unrecognized names fall back to Dracula) |
| `title` | Text displayed in the terminal title bar; `None` renders an empty bar |

Returns the encoded PNG as a `Vec<u8>`.

## Themes

| Theme | Background | Foreground |
|-------|-----------|-----------|
| `dracula` | `#282a36` | `#f8f8f2` |
| `monokai` | `#272822` | `#f8f8f2` |

Both themes include full 16-color ANSI palettes, a styled chrome bar, and traffic light buttons. Unknown theme names default to Dracula.

## Modules

| Module | Responsibility |
|--------|---------------|
| `ansi_parse` | Parse ANSI escape sequences into a `CellGrid` |
| `svg` | Render `CellGrid` to an SVG string with terminal chrome |
| `rasterize` | Rasterize an SVG string to PNG bytes via resvg |
| `themes` | Theme definitions and `get_theme` lookup |

## License

Apache-2.0
