use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "output_type", rename_all = "lowercase")]
pub enum OutputFormat {
    Png(PngConfig),
    Gif(GifConfig),
    Mp4(Mp4Config),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PngConfig {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GifConfig {
    #[serde(default = "default_gif_quality")]
    pub quality: u8,
    #[serde(default)]
    pub fast: bool,
    /// None or 0 = infinite repeat
    pub repeat: Option<u16>,
}

impl Default for GifConfig {
    fn default() -> Self {
        Self {
            quality: default_gif_quality(),
            fast: false,
            repeat: None,
        }
    }
}

fn default_gif_quality() -> u8 {
    90
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mp4Config {
    #[serde(default = "default_mp4_fps")]
    pub fps: u32,
}

impl Default for Mp4Config {
    fn default() -> Self {
        Self {
            fps: default_mp4_fps(),
        }
    }
}

fn default_mp4_fps() -> u32 {
    24
}

fn default_wait() -> u64 {
    1000
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Interaction {
    Type { text: String, speed: Option<u64> },
    Key { key: String },
    Click { selector: Option<String> },
    Hover { selector: Option<String> },
    ScrollTo { selector: Option<String> },
    Wait {
        #[serde(default = "default_wait")]
        duration: u64,
    },
    Snapshot { name: Option<String> },
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
        #[serde(default)]
        interactions: Vec<Interaction>,
        frame_duration: Option<u64>,
        #[serde(default)]
        full_page: Option<bool>,
    },
    Screen {
        name: Option<String>,
        display: Option<u32>,
        window: Option<String>,
        region: Option<Region>,
        formats: Option<Vec<OutputFormat>>,
        setup: Option<String>,
        delay: Option<u64>,
        title: Option<String>,
        #[serde(default = "default_theme")]
        theme: String,
        #[serde(default)]
        interactions: Vec<Interaction>,
        frame_duration: Option<u64>,
    },
    Terminal {
        name: Option<String>,
        formats: Option<Vec<OutputFormat>>,
        theme: Option<String>,
        cols: Option<usize>,
        rows: Option<usize>,
        #[serde(default)]
        interactions: Vec<Interaction>,
        frame_duration: Option<u64>,
    },
}

impl SceneConfig {
    pub fn name(&self) -> &str {
        match self {
            SceneConfig::Web { name, url, .. } => name.as_deref().unwrap_or(url.as_str()),
            SceneConfig::Screen { name, .. } => name.as_deref().unwrap_or("screen"),
            SceneConfig::Terminal { name, .. } => name.as_deref().unwrap_or("recording"),
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

    pub fn interactions(&self) -> &[Interaction] {
        match self {
            SceneConfig::Web { interactions, .. } => interactions,
            SceneConfig::Screen { interactions, .. } => interactions,
            SceneConfig::Terminal { interactions, .. } => interactions,
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

fn default_theme() -> String {
    "dracula".to_string()
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
    vec![OutputFormat::Png(PngConfig::default())]
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
    /// Frames per second (converted to frame_duration_ms = 1000/fps).
    pub fps: Option<u32>,
    /// Per-scene capture timeout in seconds.
    pub seconds: Option<f64>,
}

/// Fully resolved config with defaults applied.
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub scenes: Vec<SceneConfig>,
    pub server: Option<ServerConfig>,
    pub viewport: ViewportConfig,
    pub output: OutputConfig,
    /// Global frame duration in ms, derived from fps (default: 24fps → 41ms).
    pub frame_duration_ms: u64,
    /// Per-scene capture timeout in seconds (default: 2.5s).
    pub seconds: f64,
}

impl TeaseConfig {
    pub fn resolve(self) -> ResolvedConfig {
        ResolvedConfig {
            scenes: self.scenes,
            server: self.server,
            viewport: self.viewport.unwrap_or_default(),
            output: self.output.unwrap_or_default(),
            frame_duration_ms: 1000 / self.fps.unwrap_or(24).max(1) as u64,
            seconds: self.seconds.unwrap_or(2.5),
        }
    }
}

/// A single captured frame with PNG data and its display duration.
pub struct CapturedFrame {
    pub png_data: Vec<u8>,
    pub duration_ms: u64,
}

#[derive(Debug)]
pub struct CaptureResult {
    pub scene_name: String,
    pub files: Vec<String>,
}
