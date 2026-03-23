use anyhow::{Context, Result};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tracing::debug;

use crate::backend::CaptureBackend;
use crate::types::{CapturedFrame, Interaction};

pub struct TerminalBackend {
    cols: usize,
    rows: Option<usize>,
    theme: String,
    title: Option<String>,
    frame_duration: u64,
    cwd: Option<String>,
    font_family: Option<String>,
    font_size: Option<f64>,
    writer: Option<Box<dyn std::io::Write + Send>>,
    buffer: Option<Arc<Mutex<Vec<u8>>>>,
    emulator: Option<teasr_term_render::TerminalEmulator>,
    reader_handle: Option<JoinHandle<()>>,
    child: Option<Box<dyn portable_pty::Child + Send>>,
}

impl TerminalBackend {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cols: usize,
        rows: Option<usize>,
        theme: &str,
        title: Option<String>,
        frame_duration: u64,
        cwd: Option<String>,
        font_family: Option<String>,
        font_size: Option<f64>,
    ) -> Self {
        Self {
            cols,
            rows,
            theme: theme.to_string(),
            title,
            frame_duration,
            cwd,
            font_family,
            font_size,
            writer: None,
            buffer: None,
            emulator: None,
            reader_handle: None,
            child: None,
        }
    }

    fn drain_and_snapshot(&mut self) -> Result<Vec<u8>> {
        let buffer = self.buffer.as_ref().unwrap();
        let emulator = self.emulator.as_mut().unwrap();
        let data: Vec<u8> = {
            let mut lock = buffer.lock().unwrap();
            let d = lock.clone();
            lock.clear();
            d
        };
        if !data.is_empty() {
            emulator.feed(&data);
        }
        let grid = emulator.snapshot();
        let opts = teasr_term_render::RenderOptions {
            theme_name: &self.theme,
            title: self.title.as_deref(),
            font_family: self.font_family.as_deref(),
            font_size: self.font_size,
        };
        teasr_term_render::render_grid_to_png(&grid, &opts)
    }
}

#[async_trait::async_trait]
impl CaptureBackend for TerminalBackend {
    fn mode_name(&self) -> &'static str {
        "terminal"
    }

    async fn setup(&mut self) -> Result<()> {
        let pty_system = native_pty_system();
        let pty_rows = self.rows.unwrap_or(500) as u16;
        let pair = pty_system
            .openpty(PtySize {
                rows: pty_rows,
                cols: self.cols as u16,
                pixel_width: 0,
                pixel_height: 0,
            })
            .context("failed to open PTY")?;

        let shell = if cfg!(windows) {
            "cmd".to_string()
        } else {
            std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string())
        };
        let mut cmd = CommandBuilder::new(&shell);
        if !cfg!(windows) {
            cmd.arg("-li");
        }

        // Point shell rc dirs to an empty temp dir so startup files don't pollute
        // the recording, but keep the real HOME so tools find their auth/config.
        let rc_dir = tempfile::tempdir().context("failed to create temp rc dir")?;
        let rc_path = rc_dir.path().to_str().unwrap_or("/tmp");
        cmd.env("ZDOTDIR", rc_path); // zsh reads rc from ZDOTDIR instead of HOME
        cmd.env("BASH_ENV", "");
        cmd.env("ENV", ""); // sh/dash
        cmd.env("HISTFILE", "/dev/null");
        cmd.env("TERM", "xterm-256color");
        cmd.env("FORCE_COLOR", "1");
        cmd.env("COLORTERM", "truecolor");
        cmd.env("PS1", "$ ");
        cmd.env("TERM_PROGRAM", "");

        let child = pair
            .slave
            .spawn_command(cmd)
            .context("failed to spawn shell")?;
        drop(pair.slave);

        let writer = pair
            .master
            .take_writer()
            .context("failed to get PTY writer")?;
        let mut reader = pair
            .master
            .try_clone_reader()
            .context("failed to get PTY reader")?;

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

        self.writer = Some(writer);
        self.buffer = Some(buffer);
        self.emulator = Some(if let Some(rows) = self.rows {
            teasr_term_render::TerminalEmulator::new(self.cols, rows)
        } else {
            teasr_term_render::TerminalEmulator::new_unbounded(self.cols)
        });
        self.reader_handle = Some(reader_handle);
        self.child = Some(child);

        // Wait for shell prompt
        thread::sleep(Duration::from_millis(200));

        // Resolve the target working directory
        let abs_cwd = {
            let cwd = self.cwd.clone().unwrap_or_else(|| ".".to_string());
            let p = std::path::Path::new(&cwd);
            if p.is_relative() {
                std::env::current_dir()
                    .context("failed to get current dir")?
                    .join(p)
            } else {
                p.to_path_buf()
            }
        };

        // Copy the cwd to a temp dir so the demo can't break the real files
        let effective_cwd = {
            let copy_dir = tempfile::tempdir().context("failed to create copy dir")?;
            let copy_path = copy_dir.path().join("repo");
            let status = std::process::Command::new("cp")
                .args(["-a"])
                .arg(&abs_cwd)
                .arg(&copy_path)
                .status()
                .context("failed to copy working directory")?;
            if !status.success() {
                anyhow::bail!("cp -a failed (exit {})", status);
            }
            debug!("copied {} -> {}", abs_cwd.display(), copy_path.display());
            // Leak the tempdir so it lives until process exit
            std::mem::forget(copy_dir);
            copy_path
        };

        // cd into the effective cwd and clear the screen
        {
            use std::io::Write;
            let writer = self.writer.as_mut().unwrap();
            writer
                .write_all(
                    format!(
                        "cd {} && clear\n",
                        shell_escape(&effective_cwd.to_string_lossy())
                    )
                    .as_bytes(),
                )
                .context("failed to cd into cwd")?;
            thread::sleep(Duration::from_millis(500));
            // Drain the buffer so the cd/clone doesn't appear in output
            if let Some(ref buffer) = self.buffer {
                buffer.lock().unwrap().clear();
            }
            if let Some(ref mut emulator) = self.emulator {
                *emulator = if let Some(rows) = self.rows {
                    teasr_term_render::TerminalEmulator::new(self.cols, rows)
                } else {
                    teasr_term_render::TerminalEmulator::new_unbounded(self.cols)
                };
            }
        }

        // Keep the tempdir alive by leaking it (it'll be cleaned up on process exit)
        std::mem::forget(rc_dir);

        Ok(())
    }

    async fn execute(&mut self, interaction: &Interaction) -> Result<Vec<CapturedFrame>> {
        match interaction {
            Interaction::Type { text, speed } => {
                let char_delay = Duration::from_millis(speed.unwrap_or(50));
                let mut frames = Vec::new();
                for ch in text.chars() {
                    let mut bytes = [0u8; 4];
                    let s = ch.encode_utf8(&mut bytes);
                    self.writer
                        .as_mut()
                        .unwrap()
                        .write_all(s.as_bytes())
                        .context("failed to write to PTY")?;
                    thread::sleep(char_delay);
                    thread::sleep(Duration::from_millis(10));
                    frames.push(CapturedFrame {
                        png_data: self.drain_and_snapshot()?,
                        duration_ms: self.frame_duration,
                    });
                }
                Ok(frames)
            }
            Interaction::Key { key } => {
                let bytes = key_to_bytes(key);
                self.writer
                    .as_mut()
                    .unwrap()
                    .write_all(&bytes)
                    .context("failed to write key to PTY")?;
                thread::sleep(Duration::from_millis(50));
                Ok(vec![CapturedFrame {
                    png_data: self.drain_and_snapshot()?,
                    duration_ms: self.frame_duration,
                }])
            }
            Interaction::Wait { duration } => {
                let interval = self.frame_duration.max(50);
                let steps = (*duration / interval).max(1);
                let step_ms = *duration / steps;
                let mut frames = Vec::new();
                for _ in 0..steps {
                    thread::sleep(Duration::from_millis(step_ms));
                    frames.push(CapturedFrame {
                        png_data: self.drain_and_snapshot()?,
                        duration_ms: step_ms,
                    });
                }
                Ok(frames)
            }
            Interaction::Snapshot { .. } => {
                Ok(vec![CapturedFrame {
                    png_data: self.drain_and_snapshot()?,
                    duration_ms: self.frame_duration,
                }])
            }
            other => {
                debug!(
                    "skipping unsupported interaction: {:?} ({})",
                    other,
                    self.mode_name()
                );
                Ok(vec![])
            }
        }
    }

    async fn snapshot(&mut self) -> Result<CapturedFrame> {
        Ok(CapturedFrame {
            png_data: self.drain_and_snapshot()?,
            duration_ms: self.frame_duration,
        })
    }

    async fn teardown(&mut self) -> Result<()> {
        if let Some(ref mut writer) = self.writer {
            let _ = writer.write_all(b"exit\n");
        }
        self.writer = None;
        if let Some(mut child) = self.child.take() {
            let _ = child.wait();
        }
        if let Some(handle) = self.reader_handle.take() {
            let _ = handle.join();
        }
        Ok(())
    }
}

fn shell_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
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
