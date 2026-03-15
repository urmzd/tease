---
name: teasr-dev
description: >
  Development skill for the teasr Rust workspace — a CLI tool that captures screenshots and GIFs
  from web apps, terminals, and screens. Use this skill when working on any of the three workspace
  crates (teasr-cli, teasr-core, teasr-term-render), adding capture backends, modifying the config
  system, debugging build issues, or extending output formats. Trigger whenever the user mentions
  teasr, screenshot capture, terminal rendering, ANSI parsing, or the workspace crates by name.
---

# teasr Development Guide

## Architecture at a Glance

teasr is a Cargo workspace with three crates that form a pipeline:

```
teasr-cli (binary)
  └─ teasr-core (library)
       ├─ config       → TOML parsing, file auto-discovery
       ├─ orchestrator → scene loop, format dispatch
       ├─ server       → RAII dev server lifecycle
       ├─ capture/
       │    ├─ web.rs       → Chrome DevTools Protocol (chromiumoxide)
       │    ├─ terminal.rs  → PTY capture (portable-pty)
       │    └─ screen.rs    → native screenshot (xcap)
       └─ convert/
            └─ gif.rs       → PNG→GIF (gifski)
  └─ teasr-term-render (library)
       ├─ ansi_parse.rs → ANSI escape sequence → cell grid
       ├─ svg.rs        → cell grid → SVG with terminal chrome
       ├─ rasterize.rs  → SVG → PNG (resvg + tiny-skia)
       └─ themes.rs     → Dracula, Monokai color themes
```

## Build & Test Commands

```bash
# Build everything
cargo build --workspace

# Run all tests
cargo test --workspace

# Lint (CI enforces zero warnings)
cargo clippy --workspace -- -D warnings

# Run the CLI locally
cargo run -p teasr-cli -- [--verbose] [--formats png,gif] [-o ./out]

# Update snapshot tests (teasr-term-render)
cargo insta review
```

## Key Patterns

### Error Handling
Use `anyhow::Result` everywhere. No custom error types. Attach context with `.context("what failed")`.

### Logging
Use `tracing` macros (`tracing::info!`, `tracing::debug!`, etc.). Never `println!` or `eprintln!` in library crates.

### Dependencies
Shared deps live in workspace `Cargo.toml` under `[workspace.dependencies]`. Crates inherit with `dep.workspace = true`. Add new shared deps at the workspace level first.

### Platform-Specific Code
Gate with `#[cfg(unix)]` / `#[cfg(windows)]`. See `server.rs` for the pattern — Unix uses `nix` for process groups and signals, Windows uses `TerminateProcess`.

### Adding a New Capture Backend
1. Create `crates/teasr-core/src/capture/<name>.rs`
2. Add a variant to `SceneConfig` enum in `types.rs`
3. Add config deserialization support in `types.rs`
4. Wire it into the match in `orchestrator.rs`
5. Re-export from `capture/mod.rs`

### Adding a New Output Format
1. Add variant to `OutputFormat` enum in `types.rs`
2. Create `crates/teasr-core/src/convert/<name>.rs`
3. Wire conversion in `orchestrator.rs`
4. Re-export from `convert/mod.rs`

### Terminal Rendering Pipeline
The rendering flows through four stages: raw ANSI bytes → parsed cell grid (`ansi_parse.rs`) → SVG string with terminal chrome (`svg.rs`) → PNG raster (`rasterize.rs`). The public API is just `render_to_png()` in `lib.rs`. Themes are defined as color palettes in `themes.rs`.

## Configuration

teasr uses TOML config (`teasr.toml`), auto-discovered by walking from cwd to filesystem root. The schema lives in `types.rs` as serde-derived structs. An example config is at the repo root.

## CI/CD

- `.github/workflows/ci.yml` — build + clippy + test on every push/PR
- `.github/workflows/release.yml` — semantic-release for 6 platform targets
- `.github/actions/tease/action.yml` — composite GitHub Action for users
- `sr.yaml` — semantic-release config (conventional commits)

## Commit Convention

`type(scope)?: description` — feat (minor), fix/perf (patch), everything else (no bump).
