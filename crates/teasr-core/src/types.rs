use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Png,
    Gif,
    Mp4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum TerminalStep {
    Type {
        text: String,
        speed: Option<u64>,
    },
    Key {
        key: String,
    },
    Wait {
        duration: Option<u64>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewportConfig {
    #[serde(default = "default_width")]
    pub width: u32,
    #[serde(default = "default_height")]
    pub height: u32,
}

fn default_width() -> u32 {
    1280
}
fn default_height() -> u32 {
    720
}

impl Default for ViewportConfig {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ActionType {
    ScrollTo,
    Click,
    Wait,
    Hover,
    Screenshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureAction {
    #[serde(rename = "type")]
    pub action_type: ActionType,
    pub selector: Option<String>,
    pub delay: Option<u64>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SceneConfig {
    Web {
        url: String,
        name: Option<String>,
        viewport: Option<ViewportConfig>,
        formats: Option<Vec<OutputFormat>>,
        actions: Option<Vec<CaptureAction>>,
    },
    Screen {
        name: Option<String>,
        display: Option<u32>,
        region: Option<Region>,
        formats: Option<Vec<OutputFormat>>,
        setup: Option<String>,
        delay: Option<u64>,
    },
    Terminal {
        name: Option<String>,
        formats: Option<Vec<OutputFormat>>,
        theme: Option<String>,
        cols: Option<usize>,
        rows: Option<usize>,
        steps: Vec<TerminalStep>,
        frame_duration: Option<u64>,
    },
}

impl SceneConfig {
    pub fn name(&self) -> &str {
        match self {
            SceneConfig::Web { name, url, .. } => {
                name.as_deref().unwrap_or(url.as_str())
            }
            SceneConfig::Screen { name, .. } => name.as_deref().unwrap_or("screen"),
            SceneConfig::Terminal { name, .. } => {
                name.as_deref().unwrap_or("recording")
            }
        }
    }

    pub fn formats(&self) -> &Option<Vec<OutputFormat>> {
        match self {
            SceneConfig::Web { formats, .. } => formats,
            SceneConfig::Screen { formats, .. } => formats,
            SceneConfig::Terminal { formats, .. } => formats,
        }
    }

    pub fn scene_type(&self) -> &str {
        match self {
            SceneConfig::Web { .. } => "web",
            SceneConfig::Screen { .. } => "screen",
            SceneConfig::Terminal { .. } => "terminal",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub command: String,
    pub url: String,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

fn default_timeout() -> u64 {
    10000
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    #[serde(default = "default_output_dir")]
    pub dir: String,
    #[serde(default = "default_formats")]
    pub formats: Vec<OutputFormat>,
}

fn default_output_dir() -> String {
    "./teasr-output".to_string()
}

fn default_formats() -> Vec<OutputFormat> {
    vec![OutputFormat::Png]
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            dir: default_output_dir(),
            formats: default_formats(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeaseConfig {
    pub scenes: Vec<SceneConfig>,
    pub server: Option<ServerConfig>,
    pub viewport: Option<ViewportConfig>,
    pub output: Option<OutputConfig>,
}

/// Fully resolved config with defaults applied.
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub scenes: Vec<SceneConfig>,
    pub server: Option<ServerConfig>,
    pub viewport: ViewportConfig,
    pub output: OutputConfig,
}

impl TeaseConfig {
    pub fn resolve(self) -> ResolvedConfig {
        ResolvedConfig {
            scenes: self.scenes,
            server: self.server,
            viewport: self.viewport.unwrap_or_default(),
            output: self.output.unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct CaptureResult {
    pub scene_name: String,
    pub files: Vec<String>,
}
