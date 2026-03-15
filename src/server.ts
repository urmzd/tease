import { spawn, type ChildProcess } from "node:child_process";
import { log } from "./logger.js";
import type { ServerConfig } from "./types.js";

async function pollUrl(url: string, timeout: number): Promise<void> {
  const start = Date.now();
  const interval = 500;

  while (Date.now() - start < timeout) {
    try {
      const response = await fetch(url);
      if (response.ok) {
        return;
      }
    } catch {
      // Server not ready yet
    }
    await new Promise((r) => setTimeout(r, interval));
  }

  throw new Error(`Server at ${url} did not respond within ${timeout}ms`);
}

export async function startServer(
  config: ServerConfig,
): Promise<() => void> {
  const timeout = config.timeout ?? 30_000;
  const [cmd, ...args] = config.command.split(" ");

  log.info(`Starting server: ${config.command}`);

  const proc: ChildProcess = spawn(cmd, args, {
    stdio: "pipe",
    shell: true,
    env: { ...process.env, FORCE_COLOR: "0" },
  });

  proc.stderr?.on("data", (data: Buffer) => {
    log.debug(`[server] ${data.toString().trim()}`);
  });

  proc.stdout?.on("data", (data: Buffer) => {
    log.debug(`[server] ${data.toString().trim()}`);
  });

  log.info(`Waiting for ${config.url} to be ready...`);
  await pollUrl(config.url, timeout);
  log.success("Server is ready.");

  return () => {
    if (!proc.killed) {
      proc.kill("SIGTERM");
      log.debug("Server process terminated.");
    }
  };
}
