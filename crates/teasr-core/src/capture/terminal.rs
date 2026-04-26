use anyhow::{Context, Result};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
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
    command: Option<String>,
    font_family: Option<String>,
    font_size: Option<f64>,
    writer: Option<Box<dyn std::io::Write + Send>>,
    buffer: Option<Arc<Mutex<Vec<u8>>>>,
    emulator: Option<crate::term_render::TerminalEmulator>,
    reader_handle: Option<JoinHandle<()>>,
    child: Option<Box<dyn portable_pty::Child + Send>>,
    last_grid: Option<crate::term_render::CellGrid>,
    last_png: Option<Vec<u8>>,
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
        command: Option<String>,
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
            command,
            font_family,
            font_size,
            writer: None,
            buffer: None,
            emulator: None,
            reader_handle: None,
            child: None,
            last_grid: None,
            last_png: None,
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
        let mut grid = emulator.snapshot();

        // Normalize the grid to a fixed viewport so every frame has consistent
        // dimensions. For bounded emulators use the configured row count; for
        // unbounded ones default to 24 rows (matching intro/outro splash size).
        let viewport_rows = self.rows.unwrap_or(24);
        let cols = grid.cols;
        if grid.rows.len() > viewport_rows {
            let start = grid.rows.len() - viewport_rows;
            grid.rows = grid.rows.split_off(start);
        } else {
            let empty_row = vec![crate::term_render::Cell::default(); cols];
            while grid.rows.len() < viewport_rows {
                grid.rows.push(empty_row.clone());
            }
        }

        // Skip re-rendering if the grid hasn't changed
        if let Some(ref last) = self.last_grid {
            if *last == grid {
                return Ok(self.last_png.clone().unwrap());
            }
        }

        let opts = crate::term_render::RenderOptions {
            theme_name: &self.theme,
            title: self.title.as_deref(),
            font_family: self.font_family.as_deref(),
            font_size: self.font_size,
        };
        let png = crate::term_render::render_grid_to_png(&grid, &opts)?;
        self.last_grid = Some(grid);
        self.last_png = Some(png.clone());
        Ok(png)
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

        // Resolve the target working directory
        let effective_cwd = {
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

        let shell = if cfg!(windows) {
            "cmd".to_string()
        } else {
            std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string())
        };

        let rc_dir = tempfile::tempdir().context("failed to create temp rc dir")?;
        let rc_path = rc_dir.path().to_str().unwrap_or("/tmp");

        let child = if let Some(ref command) = self.command {
            // Direct spawn: shell only parses the command, exec replaces it
            // so the process is directly trackable by PID.
            let mut cmd = CommandBuilder::new(&shell);
            cmd.arg("-c");
            cmd.arg(format!("exec {command}"));
            cmd.cwd(&effective_cwd);
            cmd.env("TERM", "xterm-256color");
            cmd.env("FORCE_COLOR", "1");
            cmd.env("COLORTERM", "truecolor");
            cmd.env("TERM_PROGRAM", "");
            cmd.env("ZDOTDIR", rc_path);
            cmd.env("BASH_ENV", "");
            cmd.env("ENV", "");
            cmd.env("HISTFILE", "/dev/null");

            pair.slave
                .spawn_command(cmd)
                .context("failed to spawn command")?
        } else {
            // Interactive shell mode: spawn a login shell, cd into cwd
            let mut cmd = CommandBuilder::new(&shell);
            if !cfg!(windows) {
                cmd.arg("-li");
            }
            cmd.env("ZDOTDIR", rc_path);
            cmd.env("BASH_ENV", "");
            cmd.env("ENV", "");
            cmd.env("HISTFILE", "/dev/null");
            cmd.env("TERM", "xterm-256color");
            cmd.env("FORCE_COLOR", "1");
            cmd.env("COLORTERM", "truecolor");
            cmd.env("PS1", "$ ");
            cmd.env("TERM_PROGRAM", "");

            pair.slave
                .spawn_command(cmd)
                .context("failed to spawn shell")?
        };
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
            crate::term_render::TerminalEmulator::new(self.cols, rows)
        } else {
            crate::term_render::TerminalEmulator::new_unbounded(self.cols)
        });
        self.reader_handle = Some(reader_handle);
        self.child = Some(child);

        if self.command.is_some() {
            // Direct spawn: wait for the process to produce initial output
            // so the first snapshot captures the rendered UI. Do NOT drain
            // the buffer — for TUI apps the initial output IS the content.
            wait_for_buffer_activity(self.buffer.as_ref().unwrap(), Duration::from_millis(2000));
        } else {
            // Interactive shell: wait for prompt, cd, clear
            wait_for_buffer_match(
                self.buffer.as_ref().unwrap(),
                b"$ ",
                Duration::from_millis(2000),
            );

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
                wait_for_buffer_match(
                    self.buffer.as_ref().unwrap(),
                    b"$ ",
                    Duration::from_millis(2000),
                );
                if let Some(ref buffer) = self.buffer {
                    buffer.lock().unwrap().clear();
                }
                if let Some(ref mut emulator) = self.emulator {
                    *emulator = if let Some(rows) = self.rows {
                        crate::term_render::TerminalEmulator::new(self.cols, rows)
                    } else {
                        crate::term_render::TerminalEmulator::new_unbounded(self.cols)
                    };
                }
            }
        }

        // Keep the tempdir alive by leaking it (it'll be cleaned up on process exit)
        std::mem::forget(rc_dir);

        Ok(())
    }

    async fn execute(&mut self, interaction: &Interaction) -> Result<Vec<CapturedFrame>> {
        match interaction {
            Interaction::Type { text, speed } => {
                let base_ms = speed.unwrap_or(super::DEFAULT_TYPE_SPEED_MS);
                let mut frames = Vec::new();
                for ch in text.chars() {
                    let mut bytes = [0u8; 4];
                    let s = ch.encode_utf8(&mut bytes);
                    self.writer
                        .as_mut()
                        .unwrap()
                        .write_all(s.as_bytes())
                        .context("failed to write to PTY")?;
                    thread::sleep(super::humanized_keystroke_delay(base_ms));
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
                thread::sleep(Duration::from_millis(*duration));
                Ok(vec![CapturedFrame {
                    png_data: self.drain_and_snapshot()?,
                    duration_ms: *duration,
                }])
            }
            Interaction::Snapshot { .. } => Ok(vec![CapturedFrame {
                png_data: self.drain_and_snapshot()?,
                duration_ms: self.frame_duration,
            }]),
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
        // Try a graceful exit first
        if let Some(ref mut writer) = self.writer {
            let _ = writer.write_all(b"exit\n");
        }
        self.writer = None;

        if let Some(mut child) = self.child.take() {
            // Give the child a moment to exit gracefully, then kill it
            let deadline = Instant::now() + Duration::from_secs(2);
            loop {
                match child.try_wait() {
                    Ok(Some(_)) => break,
                    Ok(None) if Instant::now() >= deadline => {
                        debug!("child did not exit in time, killing");
                        let _ = child.kill();
                        let _ = child.wait();
                        break;
                    }
                    Ok(None) => thread::sleep(Duration::from_millis(50)),
                    Err(_) => break,
                }
            }
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

/// Poll a shared buffer until any output arrives, then wait for it to settle.
fn wait_for_buffer_activity(buffer: &Arc<Mutex<Vec<u8>>>, timeout: Duration) {
    let deadline = Instant::now() + timeout;
    // Wait for first output
    loop {
        thread::sleep(Duration::from_millis(10));
        if !buffer.lock().unwrap().is_empty() {
            break;
        }
        if Instant::now() >= deadline {
            return;
        }
    }
    // Wait for output to settle (no new data for 200ms)
    let mut last_len = 0;
    let mut idle_since = Instant::now();
    while Instant::now() < deadline {
        thread::sleep(Duration::from_millis(50));
        let current_len = buffer.lock().unwrap().len();
        if current_len != last_len {
            last_len = current_len;
            idle_since = Instant::now();
        } else if idle_since.elapsed() >= Duration::from_millis(200) {
            return;
        }
    }
}

/// Poll a shared buffer for a byte pattern, returning once found or on timeout.
fn wait_for_buffer_match(buffer: &Arc<Mutex<Vec<u8>>>, pattern: &[u8], timeout: Duration) {
    let deadline = Instant::now() + timeout;
    loop {
        thread::sleep(Duration::from_millis(10));
        {
            let lock = buffer.lock().unwrap();
            if lock.windows(pattern.len()).any(|w| w == pattern) {
                return;
            }
        }
        if Instant::now() >= deadline {
            return;
        }
    }
}
