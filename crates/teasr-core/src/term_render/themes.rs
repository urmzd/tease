/// Terminal color themes for rendering.

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    pub background: &'static str,
    pub foreground: &'static str,
    pub cursor: &'static str,
    /// ANSI colors 0-15 (standard 8 + bright 8)
    pub ansi: [&'static str; 16],
    /// Title bar background
    pub chrome_bg: &'static str,
    /// Title bar button colors [close, minimize, maximize]
    pub chrome_buttons: [&'static str; 3],
}

pub static DRACULA: Theme = Theme {
    name: "dracula",
    background: "#282a36",
    foreground: "#f8f8f2",
    cursor: "#f8f8f2",
    ansi: [
        "#21222c", // black
        "#ff5555", // red
        "#50fa7b", // green
        "#f1fa8c", // yellow
        "#bd93f9", // blue
        "#ff79c6", // magenta
        "#8be9fd", // cyan
        "#f8f8f2", // white
        "#6272a4", // bright black
        "#ff6e6e", // bright red
        "#69ff94", // bright green
        "#ffffa5", // bright yellow
        "#d6acff", // bright blue
        "#ff92df", // bright magenta
        "#a4ffff", // bright cyan
        "#ffffff", // bright white
    ],
    chrome_bg: "#1e1f29",
    chrome_buttons: ["#ff5555", "#f1fa8c", "#50fa7b"],
};

pub static MONOKAI: Theme = Theme {
    name: "monokai",
    background: "#272822",
    foreground: "#f8f8f2",
    cursor: "#f8f8f2",
    ansi: [
        "#272822", // black
        "#f92672", // red
        "#a6e22e", // green
        "#f4bf75", // yellow
        "#66d9ef", // blue
        "#ae81ff", // magenta
        "#a1efe4", // cyan
        "#f8f8f2", // white
        "#75715e", // bright black
        "#f92672", // bright red
        "#a6e22e", // bright green
        "#f4bf75", // bright yellow
        "#66d9ef", // bright blue
        "#ae81ff", // bright magenta
        "#a1efe4", // bright cyan
        "#f9f8f5", // bright white
    ],
    chrome_bg: "#1e1f1c",
    chrome_buttons: ["#f92672", "#f4bf75", "#a6e22e"],
};

pub fn get_theme(name: &str) -> &'static Theme {
    match name.to_lowercase().as_str() {
        "monokai" => &MONOKAI,
        _ => &DRACULA,
    }
}
