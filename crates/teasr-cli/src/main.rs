use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "teasr", about = "Capture showcase screenshots and GIFs", version)]
struct Cli {
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
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let filter = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| filter.into()),
        )
        .with_target(false)
        .init();

    let timeout = std::time::Duration::from_millis(cli.timeout);

    let result = tokio::time::timeout(timeout, run(cli)).await;

    match result {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(e),
        Err(_) => {
            anyhow::bail!("teasr timed out after {}ms", timeout.as_millis());
        }
    }
}

async fn run(cli: Cli) -> Result<()> {
    let config_path = if let Some(path) = &cli.config {
        path.clone()
    } else {
        let cwd = std::env::current_dir().context("failed to get cwd")?;
        teasr_core::config::discover_config(&cwd)
            .context("no teasr.toml found (searched from cwd to root). Use --config to specify.")?
    };

    info!("using config: {}", config_path.display());
    let mut config = teasr_core::config::load_config(&config_path)?;

    if let Some(output) = &cli.output {
        config.output.dir = output.clone();
    }

    if let Some(formats) = &cli.formats {
        let parsed: Vec<teasr_core::types::OutputFormat> = formats
            .iter()
            .map(|f| match f.as_str() {
                "png" => Ok(teasr_core::types::OutputFormat::Png),
                "gif" => Ok(teasr_core::types::OutputFormat::Gif),
                "mp4" => Ok(teasr_core::types::OutputFormat::Mp4),
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
