use anyhow::{Context, Result};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::Read;
use std::path::Path;

/// Capture terminal command output and render to PNG.
pub fn capture(
    command: &str,
    cols: usize,
    theme: &str,
    title: Option<&str>,
    output_path: &Path,
) -> Result<()> {
    let raw_output = run_in_pty(command, cols)?;
    let png_data = teasr_term_render::render_to_png(&raw_output, cols, theme, title)?;
    std::fs::write(output_path, &png_data)
        .with_context(|| format!("failed to write {}", output_path.display()))?;
    Ok(())
}

/// Run a command in a PTY and capture its raw output bytes.
fn run_in_pty(command: &str, cols: usize) -> Result<Vec<u8>> {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: 100,
            cols: cols as u16,
            pixel_width: 0,
            pixel_height: 0,
        })
        .context("failed to open PTY")?;

    let mut cmd = CommandBuilder::new(if cfg!(windows) { "cmd" } else { "sh" });
    cmd.arg(if cfg!(windows) { "/c" } else { "-c" });
    cmd.arg(command);
    cmd.env("TERM", "xterm-256color");
    cmd.env("FORCE_COLOR", "1");
    cmd.env("COLORTERM", "truecolor");

    let mut child = pair.slave.spawn_command(cmd).context("failed to spawn command")?;
    // Drop slave so reads on master will see EOF after child exits
    drop(pair.slave);

    let mut reader = pair.master.try_clone_reader().context("failed to get PTY reader")?;
    let mut output = Vec::new();
    reader.read_to_end(&mut output).ok(); // EOF expected

    let _status = child.wait().context("failed to wait for child")?;
    drop(pair.master);

    Ok(output)
}
