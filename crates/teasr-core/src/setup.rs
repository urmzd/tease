use anyhow::{bail, Context, Result};
use crossterm::style::Stylize;
use std::io::Read;
use std::path::{Path, PathBuf};
use tracing::debug;

use crate::ui;

/// A font available for installation.
pub struct FontEntry {
    pub name: &'static str,
    pub description: &'static str,
    pub license: &'static str,
    pub url: &'static str,
}

/// Registry of known fonts that can be installed.
pub const FONT_REGISTRY: &[FontEntry] = &[
    FontEntry {
        name: "JetBrainsMono Nerd Font",
        description: "JetBrains Mono with Nerd Font icons, box-drawing, and Powerline glyphs",
        license: "OFL-1.1 (SIL Open Font License)",
        url: "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/JetBrainsMono.zip",
    },
    FontEntry {
        name: "JetBrains Mono",
        description: "JetBrains Mono typeface for developers",
        license: "Apache-2.0",
        url: "https://github.com/JetBrains/JetBrainsMono/releases/latest/download/JetBrainsMono-2.304.zip",
    },
    FontEntry {
        name: "Monaspace Nerd Font",
        description: "GitHub's Monaspace superfamily with Nerd Font icons and Powerline glyphs",
        license: "OFL-1.1 (SIL Open Font License)",
        url: "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/Monaspace.zip",
    },
];

/// Get the system font directory for the current platform.
pub fn system_font_dir() -> Result<PathBuf> {
    let home = dirs_path()?;

    #[cfg(target_os = "macos")]
    {
        Ok(home.join("Library/Fonts"))
    }

    #[cfg(target_os = "linux")]
    {
        Ok(home.join(".local/share/fonts"))
    }

    #[cfg(target_os = "windows")]
    {
        Ok(home.join("AppData/Local/Microsoft/Windows/Fonts"))
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        bail!("unsupported platform for font installation")
    }
}

fn dirs_path() -> Result<PathBuf> {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map(PathBuf::from)
        .context("could not determine home directory")
}

/// Check if a font family is available on the system.
pub fn check_font(family: &str) -> bool {
    crate::term_render::check_font_available(family)
}

/// Install a font by name from the registry.
pub async fn install_font(name: &str) -> Result<()> {
    let entry = FONT_REGISTRY
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case(name))
        .with_context(|| {
            let available: Vec<&str> = FONT_REGISTRY.iter().map(|e| e.name).collect();
            format!("unknown font: {name}. Available: {}", available.join(", "))
        })?;

    let pb = ui::spinner(&format!("installing: {} ({})", entry.name, entry.license));
    debug!("downloading from: {}", entry.url);

    let response = reqwest::get(entry.url)
        .await
        .context("failed to download font")?;

    if !response.status().is_success() {
        bail!("download failed: HTTP {}", response.status());
    }

    let bytes = response.bytes().await.context("failed to read response")?;

    let font_dir = system_font_dir()?;
    std::fs::create_dir_all(&font_dir)
        .with_context(|| format!("failed to create font dir: {}", font_dir.display()))?;

    extract_fonts(&bytes, &font_dir)?;

    // Refresh font cache on Linux
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("fc-cache").arg("-f").status();
    }

    ui::spinner_done(&pb, Some(&format!("installed to {}", font_dir.display())));
    Ok(())
}

/// Extract .ttf/.otf files from a zip archive into the target directory.
fn extract_fonts(zip_bytes: &[u8], dest: &Path) -> Result<()> {
    let cursor = std::io::Cursor::new(zip_bytes);
    let mut archive = zip::ZipArchive::new(cursor).context("failed to open zip archive")?;

    let mut count = 0;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).context("failed to read zip entry")?;
        let name = file.name().to_string();

        // Only extract font files, skip directories and non-font files
        let is_font = name.ends_with(".ttf") || name.ends_with(".otf");
        if !is_font || file.is_dir() {
            continue;
        }

        // Use just the filename, not the full path inside the zip
        let filename = Path::new(&name)
            .file_name()
            .context("invalid font filename")?;
        let dest_path = dest.join(filename);

        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .with_context(|| format!("failed to read {name}"))?;
        std::fs::write(&dest_path, &data)
            .with_context(|| format!("failed to write {}", dest_path.display()))?;

        debug!("  extracted: {}", filename.to_string_lossy());
        count += 1;
    }

    if count == 0 {
        bail!("no font files found in archive");
    }

    ui::phase_ok(&format!("{count} font files extracted"), None);
    Ok(())
}

/// List available fonts from the registry.
pub fn list_fonts() {
    ui::header("available fonts");
    let total = FONT_REGISTRY.len();
    for (i, entry) in FONT_REGISTRY.iter().enumerate() {
        let is_last = i == total - 1;
        let connector = if is_last { "└─" } else { "├─" };
        eprintln!("  {} {}", connector.dim(), entry.name.bold());
        let branch = if is_last { "  " } else { "│ " };
        eprintln!("  {}   {}", branch.dim(), entry.description.dim());
        eprintln!(
            "  {}   {}",
            branch.dim(),
            format!("License: {}", entry.license).dim()
        );
        if !is_last {
            eprintln!("  {}", "│".dim());
        }
    }
    eprintln!();
}
