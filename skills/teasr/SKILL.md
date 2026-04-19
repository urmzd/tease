---
name: teasr
description: >
  Capture screenshots and GIFs from web apps, terminals, and screens using teasr.
  Use this skill when you need to generate project showcase assets, capture terminal
  output as styled PNGs, take web page screenshots via Chrome DevTools Protocol, or
  configure teasr.toml for automated capture pipelines.
metadata:
  argument-hint: [--formats png,gif] [--output dir]
---

# teasr — Project Showcase Capture

Capture screenshots and GIFs from web apps, terminals, and screens.

## Steps

1. Ensure `teasr.toml` exists in the project root (or specify with `--config`):
   ```toml
   [output]
   dir = "./showcase"
   formats = [{ output_type = "png" }]

   [[scenes]]
   type = "terminal"
   name = "cli-help"

   [[scenes.interactions]]
   type = "type"
   text = "my-cli --help"

   [[scenes.interactions]]
   type = "key"
   key = "enter"

   [[scenes.interactions]]
   type = "wait"
   duration = 2000
   ```

2. Run: `teasr run $ARGUMENTS`

3. Output files are written to the configured output directory.

## Scene Types

### Terminal
Scripts an interactive PTY session, captures frames at each interaction, and renders them as animated GIFs or PNGs with terminal chrome.

```toml
[[scenes]]
type = "terminal"
name = "test-output"
theme = "dracula"     # or "monokai"
cols = 100
rows = 24

[[scenes.interactions]]
type = "type"
text = "cargo test 2>&1"
speed = 50

[[scenes.interactions]]
type = "key"
key = "enter"

[[scenes.interactions]]
type = "wait"
duration = 2000
```

### Web
Loads a remote URL, local file (HTML/SVG/PDF), or Markdown file via Chrome DevTools Protocol. Requires Chrome/Chromium. Type is auto-detected from `uri`.

```toml
# Remote URL (or /path if [server] is configured)
[[scenes]]
type = "web"
uri = "/dashboard"
name = "dashboard"
viewport = { width = 1440, height = 900 }

# Local HTML
[[scenes]]
type = "web"
uri = "./docs/preview.html"

# Markdown (auto-detected by .md extension)
[[scenes]]
type = "web"
uri = "./README.md"
theme = "dark"
```

### Screen
Captures a display or region using native screen capture.

```toml
[[scenes]]
type = "screen"
name = "native-app"
display = 0
setup = "open MyApp.app"
delay = 2000
```

## Server Configuration

Start a dev server before capture:

```toml
[server]
command = "npm run dev"
url = "http://localhost:3000"
timeout = 10000
```

## CLI Options

| Flag | Description |
|------|-------------|
| `-c, --config <PATH>` | Path to teasr.toml |
| `-o, --output <DIR>` | Output directory |
| `--formats <FMT,...>` | Output formats: png, gif, mp4 |
| `--verbose` | Enable debug logging |
| `--timeout <MS>` | Global timeout (default: 60000) |
| `--fps <N>` | Frames per second (overrides config) |
| `--seconds <N>` | Target output duration in seconds |
| `--scene-timeout <N>` | Per-scene wall-clock timeout in seconds |
