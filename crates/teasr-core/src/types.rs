use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    #[serde(default = "default_font_family")]
    pub family: String,
    pub path: Option<String>,
    #[serde(default = "default_font_size")]
    pub size: f64,
}

fn default_font_family() -> String {
    "monospace".to_string()
}

fn default_font_size() -> f64 {
    14.0
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            family: default_font_family(),
            path: None,
            size: default_font_size(),
        }
    }
}

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
    30000
}

fn default_idle_timeout() -> u64 {
    500
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Interaction {
    Type {
        text: String,
        speed: Option<u64>,
    },
    Key {
        key: String,
    },
    Click {
        selector: Option<String>,
    },
    Hover {
        selector: Option<String>,
    },
    ScrollTo {
        selector: Option<String>,
    },
    Wait {
        /// Maximum time to wait in ms (default: 30000). Acts as a timeout —
        /// the wait exits early once terminal output has settled.
        #[serde(default = "default_wait")]
        duration: u64,
        /// Exit early if no new output arrives for this many ms (default: 500).
        /// Set to 0 to disable idle detection and always wait the full duration.
        #[serde(default = "default_idle_timeout")]
        idle_timeout: u64,
    },
    Snapshot {
        name: Option<String>,
    },
}

/// Wraps an Interaction with metadata. Hidden steps execute but produce no visible frames.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionStep {
    #[serde(flatten)]
    pub interaction: Interaction,
    #[serde(default)]
    pub hidden: bool,
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
        interactions: Vec<InteractionStep>,
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
        interactions: Vec<InteractionStep>,
        frame_duration: Option<u64>,
    },
    Terminal {
        name: Option<String>,
        formats: Option<Vec<OutputFormat>>,
        theme: Option<String>,
        cols: Option<usize>,
        rows: Option<usize>,
        cwd: Option<String>,
        font: Option<FontConfig>,
        intro: Option<SplashConfig>,
        outro: Option<SplashConfig>,
        #[serde(default)]
        interactions: Vec<InteractionStep>,
        frame_duration: Option<u64>,
    },
    /// Capture a local file (HTML, PDF, SVG, etc.) rendered in a headless browser.
    File {
        path: String,
        name: Option<String>,
        viewport: Option<ViewportConfig>,
        formats: Option<Vec<OutputFormat>>,
        /// Page number to capture (1-indexed, default: 1). Only applies to PDFs.
        #[serde(default = "default_page")]
        page: u32,
        frame_duration: Option<u64>,
    },
    /// Render a Markdown file as a styled HTML page and capture it.
    Markdown {
        path: String,
        name: Option<String>,
        viewport: Option<ViewportConfig>,
        formats: Option<Vec<OutputFormat>>,
        /// Markdown parsing flavor: github (default), commonmark, or custom.
        #[serde(default)]
        flavor: MarkdownFlavor,
        /// Color theme: "light" (default) or "dark".
        #[serde(default = "default_markdown_theme")]
        theme: MarkdownTheme,
        /// Path to a custom CSS file applied after the default styles.
        stylesheet: Option<String>,
        /// Path to a full HTML template with `{{content}}` placeholder.
        template: Option<String>,
        /// Capture the full rendered page instead of just the viewport.
        #[serde(default)]
        full_page: bool,
        frame_duration: Option<u64>,
        #[serde(default)]
        interactions: Vec<InteractionStep>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplashConfig {
    /// Inline text to display (supports \n for newlines)
    pub text: Option<String>,
    /// Path to a .txt or .ans file with ASCII art
    pub file: Option<String>,
    /// Path to a PNG/SVG image to overlay
    pub image: Option<String>,
    /// Duration in ms to show the splash frame
    #[serde(default = "default_splash_duration")]
    pub duration: u64,
    /// Center the content horizontally
    #[serde(default = "default_true")]
    pub center: bool,
    /// Vertical alignment of splash content: "top", "center", or "bottom"
    #[serde(default)]
    pub vertical_align: VerticalAlign,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VerticalAlign {
    #[default]
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarkdownFlavor {
    #[default]
    Github,
    Commonmark,
    Custom,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarkdownTheme {
    #[default]
    Light,
    Dark,
}

fn default_markdown_theme() -> MarkdownTheme {
    MarkdownTheme::default()
}

fn default_page() -> u32 {
    1
}

fn default_splash_duration() -> u64 {
    2000
}

fn default_true() -> bool {
    true
}

impl SceneConfig {
    pub fn name(&self) -> &str {
        match self {
            SceneConfig::Web { name, url, .. } => name.as_deref().unwrap_or(url.as_str()),
            SceneConfig::Screen { name, .. } => name.as_deref().unwrap_or("screen"),
            SceneConfig::Terminal { name, .. } => name.as_deref().unwrap_or("recording"),
            SceneConfig::File { name, path, .. } => name.as_deref().unwrap_or(path.as_str()),
            SceneConfig::Markdown { name, path, .. } => name.as_deref().unwrap_or(path.as_str()),
        }
    }

    pub fn formats(&self) -> &Option<Vec<OutputFormat>> {
        match self {
            SceneConfig::Web { formats, .. } => formats,
            SceneConfig::Screen { formats, .. } => formats,
            SceneConfig::Terminal { formats, .. } => formats,
            SceneConfig::File { formats, .. } => formats,
            SceneConfig::Markdown { formats, .. } => formats,
        }
    }

    pub fn scene_type(&self) -> &str {
        match self {
            SceneConfig::Web { .. } => "web",
            SceneConfig::Screen { .. } => "screen",
            SceneConfig::Terminal { .. } => "terminal",
            SceneConfig::File { .. } => "file",
            SceneConfig::Markdown { .. } => "markdown",
        }
    }

    pub fn interactions(&self) -> &[InteractionStep] {
        match self {
            SceneConfig::Web { interactions, .. } => interactions,
            SceneConfig::Screen { interactions, .. } => interactions,
            SceneConfig::Terminal { interactions, .. } => interactions,
            SceneConfig::File { .. } => &[],
            SceneConfig::Markdown { interactions, .. } => interactions,
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
    pub font: Option<FontConfig>,
    /// Frames per second (converted to frame_duration_ms = 1000/fps).
    pub fps: Option<u32>,
    /// Target output duration in seconds (controls GIF/video length).
    pub seconds: Option<f64>,
    /// Per-scene wall-clock timeout in seconds.
    pub scene_timeout: Option<f64>,
}

/// Fully resolved config with defaults applied.
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub scenes: Vec<SceneConfig>,
    pub server: Option<ServerConfig>,
    pub viewport: ViewportConfig,
    pub output: OutputConfig,
    pub font: FontConfig,
    /// Global frame duration in ms, derived from fps (default: 24fps → 41ms).
    pub frame_duration_ms: u64,
    /// Target output duration in seconds (default: 2.5s).
    pub seconds: f64,
    /// Per-scene wall-clock timeout in seconds (default: 60s).
    pub scene_timeout: f64,
}

impl TeaseConfig {
    pub fn resolve(self) -> ResolvedConfig {
        ResolvedConfig {
            scenes: self.scenes,
            server: self.server,
            viewport: self.viewport.unwrap_or_default(),
            output: self.output.unwrap_or_default(),
            font: self.font.unwrap_or_default(),
            frame_duration_ms: 1000 / self.fps.unwrap_or(24).max(1) as u64,
            seconds: self.seconds.unwrap_or(2.5),
            scene_timeout: self.scene_timeout.unwrap_or(60.0),
        }
    }
}

/// A single captured frame with PNG data and its display duration.
#[derive(Clone)]
pub struct CapturedFrame {
    pub png_data: Vec<u8>,
    pub duration_ms: u64,
}

#[derive(Debug)]
pub struct CaptureResult {
    pub scene_name: String,
    pub files: Vec<String>,
}
