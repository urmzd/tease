use anyhow::{Context, Result};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::types::TerminalStep;

/// A single captured frame with PNG data and its display duration.
pub struct CapturedFrame {
    pub png_data: Vec<u8>,
    pub duration_ms: u64,
}

/// Record an interactive terminal session and return captured frames.
pub fn capture_session(
    cols: usize,
    rows: usize,
    theme: &str,
    title: Option<&str>,
    steps: &[TerminalStep],
    frame_duration: u64,
) -> Result<Vec<CapturedFrame>> {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: rows as u16,
            cols: cols as u16,
            pixel_width: 0,
            pixel_height: 0,
        })
        .context("failed to open PTY")?;

    let mut cmd = CommandBuilder::new(if cfg!(windows) { "cmd" } else { "sh" });
    cmd.env("TERM", "xterm-256color");
    cmd.env("FORCE_COLOR", "1");
    cmd.env("COLORTERM", "truecolor");
    cmd.env("PS1", "$ ");
    cmd.env("TERM_PROGRAM", "");

    let mut child = pair.slave.spawn_command(cmd).context("failed to spawn shell")?;
    drop(pair.slave);

    let mut writer = pair.master.take_writer().context("failed to get PTY writer")?;
    let mut reader = pair.master.try_clone_reader().context("failed to get PTY reader")?;

    // Background reader thread collects PTY output
    let buffer: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
    let buf_clone = Arc::clone(&buffer);
    let reader_handle = thread::spawn(move || {
        let mut tmp = [0u8; 4096];
        loop {
            match reader.read(&mut tmp) {
                Ok(0) => break,
                Ok(n) => {
                    buf_clone.lock().unwrap().extend_from_slice(&tmp[..n]);
                }
                Err(_) => break,
            }
        }
    });

    let mut emulator = teasr_term_render::TerminalEmulator::new(cols, rows);
    let mut frames: Vec<CapturedFrame> = Vec::new();

    // Helper closure: drain buffer, feed emulator, snapshot, render PNG
    let drain_and_snapshot =
        |buf: &Arc<Mutex<Vec<u8>>>,
         emu: &mut teasr_term_render::TerminalEmulator,
         theme: &str,
         title: Option<&str>|
         -> Result<Vec<u8>> {
            let data: Vec<u8> = {
                let mut lock = buf.lock().unwrap();
                let d = lock.clone();
                lock.clear();
                d
            };
            if !data.is_empty() {
                emu.feed(&data);
            }
            let grid = emu.snapshot();
            teasr_term_render::render_grid_to_png(&grid, theme, title)
        };

    // Wait for shell prompt to appear
    thread::sleep(Duration::from_millis(200));

    // Initial frame showing the prompt
    frames.push(CapturedFrame {
        png_data: drain_and_snapshot(&buffer, &mut emulator, theme, title)?,
        duration_ms: frame_duration,
    });

    for step in steps {
        match step {
            TerminalStep::Type { text, speed } => {
                let char_delay = Duration::from_millis(speed.unwrap_or(50));
                for ch in text.chars() {
                    let mut bytes = [0u8; 4];
                    let s = ch.encode_utf8(&mut bytes);
                    writer
                        .write_all(s.as_bytes())
                        .context("failed to write to PTY")?;
                    thread::sleep(char_delay);
                    thread::sleep(Duration::from_millis(10));
                    frames.push(CapturedFrame {
                        png_data: drain_and_snapshot(&buffer, &mut emulator, theme, title)?,
                        duration_ms: frame_duration,
                    });
                }
            }
            TerminalStep::Key { key } => {
                let bytes = key_to_bytes(key);
                writer
                    .write_all(&bytes)
                    .context("failed to write key to PTY")?;
                thread::sleep(Duration::from_millis(50));
                frames.push(CapturedFrame {
                    png_data: drain_and_snapshot(&buffer, &mut emulator, theme, title)?,
                    duration_ms: frame_duration,
                });
            }
            TerminalStep::Wait { duration } => {
                let wait_ms = duration.unwrap_or(1000);
                thread::sleep(Duration::from_millis(wait_ms));
                frames.push(CapturedFrame {
                    png_data: drain_and_snapshot(&buffer, &mut emulator, theme, title)?,
                    duration_ms: wait_ms,
                });
            }
        }
    }

    // Cleanup: exit the shell
    let _ = writer.write_all(b"exit\n");
    drop(writer);
    let _ = child.wait();
    let _ = reader_handle.join();

    Ok(frames)
}

/// Write the last frame of a session as a PNG file.
pub fn write_last_frame_png(frames: &[CapturedFrame], output_path: &Path) -> Result<()> {
    let last = frames
        .last()
        .context("no frames to write")?;
    std::fs::write(output_path, &last.png_data)
        .with_context(|| format!("failed to write {}", output_path.display()))?;
    Ok(())
}

/// Convert a key name to the bytes to send to a PTY.
fn key_to_bytes(key: &str) -> Vec<u8> {
    match key.to_lowercase().as_str() {
        "enter" | "return" => vec![b'\n'],
        "tab" => vec![b'\t'],
        "escape" | "esc" => vec![0x1b],
        "backspace" => vec![0x7f],
        "ctrl-c" => vec![0x03],
        "ctrl-d" => vec![0x04],
        "ctrl-z" => vec![0x1a],
        "ctrl-l" => vec![0x0c],
        "up" => vec![0x1b, b'[', b'A'],
        "down" => vec![0x1b, b'[', b'B'],
        "right" => vec![0x1b, b'[', b'C'],
        "left" => vec![0x1b, b'[', b'D'],
        "space" => vec![b' '],
        _ => key.as_bytes().to_vec(),
    }
}
