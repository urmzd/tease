use anyhow::{Context, Result};
use std::process::Command;
use std::time::{Duration, Instant};
use tracing::{debug, info};

use crate::types::ServerConfig;

/// A managed server process with proper cleanup.
pub struct ManagedServer {
    #[cfg(unix)]
    pgid: i32,
    #[cfg(windows)]
    child: std::process::Child,
}

impl ManagedServer {
    /// Spawn a dev server and wait until it's ready.
    pub async fn start(config: &ServerConfig) -> Result<Self> {
        info!("starting server: {}", config.command);

        #[cfg(unix)]
        let server = {
            use std::os::unix::process::CommandExt;

            let child = unsafe {
                Command::new("sh")
                    .arg("-c")
                    .arg(&config.command)
                    .pre_exec(|| {
                        // Create a new process group so we can kill all descendants
                        nix::unistd::setsid().ok();
                        Ok(())
                    })
                    .stdin(std::process::Stdio::null())
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn()
                    .context("failed to spawn server")?
            };

            let pgid = child.id() as i32;
            debug!("server started with pgid {pgid}");

            ManagedServer { pgid }
        };

        #[cfg(windows)]
        let server = {
            let child = Command::new("cmd")
                .arg("/c")
                .arg(&config.command)
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .context("failed to spawn server")?;

            ManagedServer { child }
        };

        // Poll until server is ready
        let deadline = Instant::now() + Duration::from_millis(config.timeout);
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(2))
            .build()?;

        loop {
            if Instant::now() > deadline {
                drop(server);
                anyhow::bail!(
                    "server failed to respond at {} within {}ms",
                    config.url,
                    config.timeout
                );
            }

            match client.get(&config.url).send().await {
                Ok(resp) if resp.status().is_success() || resp.status().is_redirection() => {
                    info!("server ready at {}", config.url);
                    break;
                }
                _ => {
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            }
        }

        Ok(server)
    }
}

impl Drop for ManagedServer {
    fn drop(&mut self) {
        debug!("shutting down server");

        #[cfg(unix)]
        {
            // Kill the entire process group
            unsafe {
                libc::killpg(self.pgid, libc::SIGTERM);
            }
            // Give it a moment, then force kill
            std::thread::sleep(Duration::from_millis(200));
            unsafe {
                libc::killpg(self.pgid, libc::SIGKILL);
            }
        }

        #[cfg(windows)]
        {
            self.child.kill().ok();
        }
    }
}
