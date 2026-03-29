use crossterm::style::Stylize;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use std::time::Duration;

/// Print the command header: cyan bold title + dim horizontal rule.
/// All output goes to stderr.
pub fn header(cmd: &str) {
    eprintln!();
    eprintln!("  {}", cmd.cyan().bold());
    eprintln!("  {}", "─".repeat(40).dim());
    eprintln!();
}

/// Create a styled braille spinner for long-running operations.
/// Draws to stderr so stdout stays clean for data.
pub fn spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_draw_target(ProgressDrawTarget::stderr());
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("  {spinner:.cyan} {msg} {elapsed:.dim}")
            .unwrap(),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

/// Finish a spinner, replacing it with a green checkmark line.
pub fn spinner_done(pb: &ProgressBar, detail: Option<&str>) {
    let msg = pb.message();
    pb.finish_and_clear();
    phase_ok(&msg, detail);
}

/// Print a completed phase with green checkmark.
///   checkmark msg . detail
pub fn phase_ok(msg: &str, detail: Option<&str>) {
    let suffix = detail
        .map(|d| format!(" · {}", d.dim()))
        .unwrap_or_default();
    eprintln!("  {} {msg}{suffix}", "✓".green().bold());
}

/// Print a warning message with yellow warning symbol.
pub fn warn(msg: &str) {
    eprintln!("  {} {}", "⚠".yellow().bold(), msg.yellow());
}

/// Print an info message with cyan symbol.
pub fn info(msg: &str) {
    eprintln!("  {} {}", "ℹ".cyan(), msg.dim());
}

/// Print a styled error message.
pub fn error(msg: &str) {
    eprintln!("  {} {}", "✗".red().bold(), msg.red());
}

/// Print a tree item with dim box-drawing connector.
pub fn tree_item(text: &str, is_last: bool) {
    let connector = if is_last { "└─" } else { "├─" };
    eprintln!("    {} {}", connector.dim(), text);
}
