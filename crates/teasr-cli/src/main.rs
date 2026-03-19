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

        /// Per-scene capture timeout in seconds (overrides config)
        #[arg(long)]
        seconds: Option<f64>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let Some(Command::Showme {
        config,
        output,
        formats,
        verbose,
        timeout,
        fps,
        seconds,
    }) = cli.command
    else {
        Cli::parse_from(["teasr", "--help"]);
        return Ok(());
    };

    let filter = if verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| filter.into()),
        )
        .with_target(false)
        .init();

    let timeout_dur = std::time::Duration::from_millis(timeout);

    let result = tokio::time::timeout(timeout_dur, run(config, output, formats, fps, seconds)).await;

    match result {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(e),
        Err(_) => {
            anyhow::bail!("teasr timed out after {}ms", timeout_dur.as_millis());
        }
    }
}

async fn run(
    config: Option<PathBuf>,
    output: Option<String>,
    formats: Option<Vec<String>>,
    fps: Option<u32>,
    seconds: Option<f64>,
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
