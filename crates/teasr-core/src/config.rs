use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use crate::types::{ResolvedConfig, TeaseConfig};

/// Discover config file by walking up from cwd.
pub fn discover_config(start: &Path) -> Option<PathBuf> {
    let mut dir = start.to_path_buf();
    loop {
        let candidate = dir.join("teasr.toml");
        if candidate.is_file() {
            return Some(candidate);
        }
        if !dir.pop() {
            return None;
        }
    }
}

/// Load and resolve config from a path.
pub fn load_config(path: &Path) -> Result<ResolvedConfig> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let config: TeaseConfig =
        toml::from_str(&content).with_context(|| format!("failed to parse {}", path.display()))?;

    if config.scenes.is_empty() {
        anyhow::bail!("config must define at least one scene");
    }

    Ok(config.resolve())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_temp_config(content: &str) -> NamedTempFile {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f
    }

    #[test]
    fn load_minimal_config() {
        let f = write_temp_config(
            r#"
[[scenes]]
type = "terminal"
name = "demo"

[[scenes.steps]]
type = "type"
text = "echo hello"

[[scenes.steps]]
type = "key"
key = "enter"
"#,
        );
        let config = load_config(f.path()).unwrap();
        assert_eq!(config.scenes.len(), 1);
        assert_eq!(config.output.dir, "./teasr-output");
    }

    #[test]
    fn load_full_config() {
        let f = write_temp_config(
            r#"
[server]
command = "npx serve ."
url = "http://localhost:3000"
timeout = 5000

[output]
dir = "./showcase"
formats = ["png"]

[[scenes]]
type = "web"
url = "/"
name = "home"

[[scenes]]
type = "terminal"
name = "listing"
theme = "dracula"
cols = 80

[[scenes.steps]]
type = "type"
text = "ls -la"

[[scenes.steps]]
type = "key"
key = "enter"

[[scenes.steps]]
type = "wait"
duration = 500
"#,
        );
        let config = load_config(f.path()).unwrap();
        assert_eq!(config.scenes.len(), 2);
        assert!(config.server.is_some());
        assert_eq!(config.output.dir, "./showcase");
    }

    #[test]
    fn reject_empty_scenes() {
        let f = write_temp_config(
            r#"
scenes = []
"#,
        );
        let err = load_config(f.path()).unwrap_err();
        assert!(err.to_string().contains("at least one scene"));
    }
}
