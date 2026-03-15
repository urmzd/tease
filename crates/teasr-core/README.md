# teasr-core

Orchestration and capture library for teasr. Handles config loading, server lifecycle, web/terminal/screen capture, and GIF conversion.

This crate is the engine behind the `teasr` CLI. Use it directly to embed teasr capture into your own Rust programs.

## Usage

```toml
[dependencies]
teasr-core = "0.2"
tokio = { version = "1", features = ["full"] }
```

```rust
use teasr_core::{config, orchestrator};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Discover teasr.toml walking up from cwd
    let cwd = std::env::current_dir()?;
    let config_path = config::discover_config(&cwd)
        .expect("no teasr.toml found");

    let config = config::load_config(&config_path)?;
    let results = orchestrator::run(&config).await?;

    for result in &results {
        println!("{}: {:?}", result.scene_name, result.files);
    }

    Ok(())
}
```

## Config Loading

```rust
// Auto-discover: walks up from a directory to filesystem root
let path: Option<PathBuf> = config::discover_config(&start_dir);

// Load and resolve (applies defaults, validates scenes non-empty)
let config: ResolvedConfig = config::load_config(&path)?;
```

`ResolvedConfig` is the fully-defaulted version of the TOML config with all `Option` fields resolved.

## Types

### `ResolvedConfig`

```rust
pub struct ResolvedConfig {
    pub scenes: Vec<SceneConfig>,
    pub server: Option<ServerConfig>,
    pub viewport: ViewportConfig,   // default: 1280x720
    pub output: OutputConfig,       // default dir: ./teasr-output, formats: [png]
}
```

### `SceneConfig`

A tagged enum with variants `Web`, `Screen`, and `Terminal`. Mirrors the `[[scenes]]` TOML entries.

### `ServerConfig`

```rust
pub struct ServerConfig {
    pub command: String,
    pub url: String,
    pub timeout: u64,   // ms, default: 10000
}
```

### `OutputConfig`

```rust
pub struct OutputConfig {
    pub dir: String,                 // default: "./teasr-output"
    pub formats: Vec<OutputFormat>,  // default: [Png]
}
```

### `OutputFormat`

```rust
pub enum OutputFormat { Png, Gif, Mp4 }
```

### `CaptureResult`

```rust
pub struct CaptureResult {
    pub scene_name: String,
    pub files: Vec<String>,  // absolute or relative paths of written files
}
```

## Orchestrator

```rust
pub async fn orchestrator::run(config: &ResolvedConfig) -> Result<Vec<CaptureResult>>
```

Runs all scenes in declaration order:

1. Creates the output directory.
2. Starts the server (if `server` is set) and health-polls it until ready.
3. Iterates scenes, dispatching to the appropriate capture backend.
4. On drop, kills the server process group (Unix: `SIGTERM` then `SIGKILL`; Windows: `TerminateProcess`).

## Capture Backends

### Web (`capture::web`)

Uses chromiumoxide (Chrome DevTools Protocol) to navigate, execute actions (click, scroll, hover, wait), and take screenshots. Requires Chrome or Chromium on `PATH` or at a standard install location.

### Terminal (`capture::terminal`)

Runs the command in a PTY via portable-pty, collects ANSI output, and delegates to `teasr-term-render` to produce a styled PNG.

### Screen (`capture::screen`)

Captures a display or region using xcap. Supports display index selection and pixel-precise region cropping.

## GIF Conversion (`convert::gif`)

```rust
pub fn png_to_gif(png_path: &Path, gif_path: &Path) -> Result<()>
```

Converts a PNG to a single-frame GIF using gifski (pure Rust, no FFmpeg).

## Server Lifecycle

`ManagedServer` is an RAII guard that starts a shell command in its own process group and tears it down on drop.

```rust
let server = ManagedServer::start(&server_config).await?;
// server is health-polled until config.url returns 2xx/3xx
// ...captures run...
// server killed when `server` is dropped
```

On Unix the entire process group receives `SIGTERM`, then `SIGKILL` 200 ms later, preventing orphaned child processes.

## License

Apache-2.0
