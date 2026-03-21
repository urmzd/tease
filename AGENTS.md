# AGENTS.md

## Project Overview

**teasr** is a Rust CLI tool that automatically captures screenshots and GIFs from web apps, desktop environments, and terminal output. It's a single-binary tool configured via TOML, built as a Cargo workspace with three crates.

### Workspace Structure

```
crates/
  teasr-cli/        # Binary crate — CLI entry point (clap)
  teasr-core/       # Library crate — orchestration, config, capture, conversion
  teasr-term-render/ # Library crate — ANSI→SVG→PNG terminal rendering
```

### Key Modules (teasr-core)

- `orchestrator.rs` — main run loop, iterates scenes, dispatches captures
- `config.rs` — TOML parsing and config file auto-discovery
- `types.rs` — all domain types (SceneConfig, OutputFormat, ViewportConfig, etc.)
- `server.rs` — RAII managed dev server with process group cleanup
- `capture/web.rs` — Chrome DevTools Protocol via chromiumoxide
- `capture/terminal.rs` — PTY-based capture via portable-pty
- `capture/screen.rs` — native screenshot via xcap
- `convert/gif.rs` — PNG→GIF via gifski

## Setup Commands

- Install Rust toolchain: `rustup default stable && rustup component add clippy`
- Build all crates: `cargo build --workspace`
- Run the CLI: `cargo run -p teasr-cli -- [args]`

## Development Workflow

- The entry point is `crates/teasr-cli/src/main.rs`
- Config is loaded from `teasr.toml` (auto-discovered walking up from cwd)
- Web capture requires Chrome/Chromium installed on the system
- Example config exists at `teasr.toml` in the repo root
- Showcase config in `teasr.toml` captures this project's GitHub page, CLI help, and Finder window

## Testing

- Run all tests: `cargo test --workspace`
- Snapshot tests (teasr-term-render): uses `insta` — run `cargo insta review` to update snapshots
- Linting: `cargo clippy --workspace -- -D warnings`

All tests and clippy must pass before merging. CI enforces this.

## Code Style

- Edition 2021, resolver 2
- Use `anyhow` for error handling (no custom error types)
- Use `tracing` for logging (not `println!` or `log`)
- Shared dependencies are declared in the workspace `Cargo.toml` `[workspace.dependencies]` — crates inherit with `dep.workspace = true`
- Unix-specific code gated behind `#[cfg(unix)]` / `#[cfg(windows)]`
- Keep modules focused and small; capture backends are one file each

## Build and CI

- CI runs on every push to `main` and PRs (`.github/workflows/ci.yml`):
  1. `cargo build --workspace`
  2. `cargo clippy --workspace -- -D warnings`
  3. `cargo test --workspace`
- Release pipeline (`.github/workflows/release.yml`) builds for 6 targets: Linux x64/ARM64, macOS x64/ARM64, Windows x64/ARM64
- GitHub Action at `.github/actions/teasr/action.yml`
- Versioning via `urmzd/semantic-release@v1` with config in `sr.yaml`

## Commit Convention

Follows conventional commits: `type(scope)?: description`

| Type     | Bump  |
|----------|-------|
| feat     | minor |
| fix      | patch |
| perf     | patch |
| docs, refactor, chore, ci, test, build, style | none |

## PR Guidelines

- Title format: `type(scope): description`
- All CI checks must pass (build, clippy, tests)
- Keep changes focused — one concern per PR

## Troubleshooting

- **Web capture fails**: Ensure Chrome/Chromium is installed and accessible in PATH
- **Server timeout**: Increase `server.timeout` in `teasr.toml` (default 10000ms)
- **Orphaned processes**: The `ManagedServer` drop impl sends SIGTERM then SIGKILL on Unix; if you Ctrl+C during development, child processes should still be cleaned up via process groups
