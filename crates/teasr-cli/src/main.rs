use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "teasr", about = "Capture showcase screenshots and GIFs", version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Install fonts and check system readiness
    Setup {
        /// Font name to install (e.g. "JetBrainsMono Nerd Font"). Lists available if omitted.
        #[arg(long)]
        fonts: Option<Option<String>>,

        /// Check if the configured font is available
        #[arg(long)]
        check: bool,
    },
    /// Run capture scenes from teasr.toml
    Showme {
        /// Path to config file (default: search for teasr.toml)
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Output directory (overrides config)
        #[arg(short, long)]
        output: Option<String>,

        /// Output formats (comma-separated: png,gif,mp4)
        #[arg(long, value_delimiter = ',')]
        formats: Option<Vec<String>>,

        /// Enable verbose logging
        #[arg(long)]
        verbose: bool,

        /// Global timeout in milliseconds
        #[arg(long, default_value = "60000")]
        timeout: u64,

        /// Frames per second (overrides config, converted to frame_duration)
        #[arg(long)]
        fps: Option<u32>,

        /// Target output duration in seconds (overrides config)
        #[arg(long)]
        seconds: Option<f64>,

        /// Per-scene wall-clock timeout in seconds (overrides config)
        #[arg(long)]
        scene_timeout: Option<f64>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Setup { fonts, check }) => {
            tracing_subscriber::fmt()
                .with_env_filter(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| "info".into()),
                )
                .with_target(false)
                .init();

            if check {
                let family = "JetBrainsMono Nerd Font";
                if teasr_core::setup::check_font(family) {
                    info!("font '{family}' is available");
                } else {
                    anyhow::bail!("font '{family}' is NOT available. Run: teasr setup --fonts");
                }
                return Ok(());
            }

            match fonts {
                Some(Some(name)) => teasr_core::setup::install_font(&name).await?,
                Some(None) => {
                    teasr_core::setup::list_fonts();
                    println!("Install with: teasr setup --fonts \"<font name>\"");
                }
                None => {
                    teasr_core::setup::list_fonts();
                    println!("Install with: teasr setup --fonts \"<font name>\"");
                    println!("Check availability: teasr setup --check");
                }
            }
            Ok(())
        }
        Some(Command::Showme {
            config,
            output,
            formats,
            verbose,
            timeout,
            fps,
            seconds,
            scene_timeout,
        }) => {
            let filter = if verbose { "debug" } else { "info" };
            tracing_subscriber::fmt()
                .with_env_filter(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| filter.into()),
                )
                .with_target(false)
                .init();

            let timeout_dur = std::time::Duration::from_millis(timeout);
            let result = tokio::time::timeout(
                timeout_dur,
                run(config, output, formats, fps, seconds, scene_timeout),
            )
            .await;

            match result {
                Ok(Ok(())) => Ok(()),
                Ok(Err(e)) => Err(e),
                Err(_) => {
                    anyhow::bail!("teasr timed out after {}ms", timeout_dur.as_millis());
                }
            }
        }
        None => {
            Cli::parse_from(["teasr", "--help"]);
            Ok(())
        }
    }
}

async fn run(
    config: Option<PathBuf>,
    output: Option<String>,
    formats: Option<Vec<String>>,
    fps: Option<u32>,
    seconds: Option<f64>,
    scene_timeout: Option<f64>,
) -> Result<()> {
    let config_path = if let Some(path) = &config {
        path.clone()
    } else {
        let cwd = std::env::current_dir().context("failed to get cwd")?;
        teasr_core::config::discover_config(&cwd)
            .context("no teasr.toml found (searched from cwd to root). Use --config to specify.")?
    };

    info!("using config: {}", config_path.display());
    let mut config = teasr_core::config::load_config(&config_path)?;

    if let Some(output) = &output {
        config.output.dir = output.clone();
    }

    if let Some(fps) = fps {
        config.frame_duration_ms = 1000 / fps.max(1) as u64;
    }

    if let Some(secs) = seconds {
        config.seconds = secs;
    }

    if let Some(t) = scene_timeout {
        config.scene_timeout = t;
    }

    if let Some(formats) = &formats {
        let parsed: Vec<teasr_core::types::OutputFormat> = formats
            .iter()
            .map(|f: &String| match f.as_str() {
                "png" => Ok(teasr_core::types::OutputFormat::Png(Default::default())),
                "gif" => Ok(teasr_core::types::OutputFormat::Gif(Default::default())),
                "mp4" => Ok(teasr_core::types::OutputFormat::Mp4(Default::default())),
                other => anyhow::bail!("unknown format: {other}"),
            })
            .collect::<Result<_>>()?;
        config.output.formats = parsed;
    }

    let results = teasr_core::orchestrator::run(&config).await?;

    for result in &results {
        for file in &result.files {
            info!("  wrote: {file}");
        }
    }

    Ok(())
}
