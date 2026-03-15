import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { chromium } from "playwright";
import { log } from "../logger.js";
import type { CaptureResult, ResolvedConfig, TerminalScene } from "../types.js";

const THEMES: Record<string, Record<string, string>> = {
  dracula: {
    bg: "#282a36",
    fg: "#f8f8f2",
    black: "#21222c",
    red: "#ff5555",
    green: "#50fa7b",
    yellow: "#f1fa8c",
    blue: "#bd93f9",
    magenta: "#ff79c6",
    cyan: "#8be9fd",
    white: "#f8f8f2",
  },
  monokai: {
    bg: "#272822",
    fg: "#f8f8f2",
    black: "#272822",
    red: "#f92672",
    green: "#a6e22e",
    yellow: "#e6db74",
    blue: "#66d9ef",
    magenta: "#ae81ff",
    cyan: "#a1efe4",
    white: "#f8f8f2",
  },
};

function escapeHtml(text: string): string {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

function stripAnsi(text: string): string {
  // eslint-disable-next-line no-control-regex
  return text.replace(/\x1b\[[0-9;]*m/g, "");
}

function ansiToHtml(text: string, theme: Record<string, string>): string {
  const colorMap: Record<string, string> = {
    "30": theme.black,
    "31": theme.red,
    "32": theme.green,
    "33": theme.yellow,
    "34": theme.blue,
    "35": theme.magenta,
    "36": theme.cyan,
    "37": theme.white,
    "90": theme.white,
  };

  let result = "";
  let i = 0;
  const chars = text;

  while (i < chars.length) {
    if (chars[i] === "\x1b" && chars[i + 1] === "[") {
      // Parse ANSI escape
      let j = i + 2;
      while (j < chars.length && chars[j] !== "m") j++;
      const codes = chars.slice(i + 2, j).split(";");
      i = j + 1;

      for (const code of codes) {
        if (code === "0" || code === "") {
          result += "</span>";
        } else if (code === "1") {
          result += '<span style="font-weight:bold">';
        } else if (colorMap[code]) {
          result += `<span style="color:${colorMap[code]}">`;
        }
      }
    } else {
      result += escapeHtml(chars[i]);
      i++;
    }
  }

  return result;
}

async function runCommand(
  command: string,
  cols: number,
  maxLines: number,
): Promise<string> {
  // Try node-pty first for colored output
  try {
    const pty = await import("node-pty");
    return await new Promise<string>((resolve, reject) => {
      let output = "";
      const shell = process.platform === "win32" ? "cmd.exe" : "/bin/sh";
      const args = process.platform === "win32" ? ["/c", command] : ["-c", command];

      const proc = pty.spawn(shell, args, {
        name: "xterm-256color",
        cols,
        rows: maxLines,
        env: { ...process.env, TERM: "xterm-256color", FORCE_COLOR: "1" } as Record<string, string>,
      });

      proc.onData((data: string) => {
        output += data;
      });

      proc.onExit(() => {
        const lines = output.split("\n").slice(0, maxLines);
        resolve(lines.join("\n"));
      });

      setTimeout(() => {
        proc.kill();
        reject(new Error(`Command timed out: ${command}`));
      }, 30_000);
    });
  } catch {
    // Fallback to child_process
    log.debug("node-pty not available, falling back to child_process");
    const { exec } = await import("node:child_process");
    const { promisify } = await import("node:util");
    const execAsync = promisify(exec);

    try {
      const { stdout, stderr } = await execAsync(command, { timeout: 30_000 });
      const combined = stdout + stderr;
      return combined.split("\n").slice(0, maxLines).join("\n");
    } catch (err: unknown) {
      const execErr = err as { stdout?: string; stderr?: string };
      return (execErr.stdout ?? "") + (execErr.stderr ?? "");
    }
  }
}

function buildTerminalHtml(
  content: string,
  theme: Record<string, string>,
  cols: number,
): string {
  const htmlContent = ansiToHtml(content, theme);

  return `<!DOCTYPE html>
<html>
<head>
<style>
  * { margin: 0; padding: 0; box-sizing: border-box; }
  body {
    background: ${theme.bg};
    padding: 16px;
    font-family: 'SF Mono', 'Fira Code', 'JetBrains Mono', 'Cascadia Code', Menlo, Monaco, 'Courier New', monospace;
    font-size: 14px;
    line-height: 1.5;
  }
  .terminal {
    background: ${theme.bg};
    color: ${theme.fg};
    border-radius: 8px;
    overflow: hidden;
    border: 1px solid rgba(255,255,255,0.1);
  }
  .titlebar {
    background: rgba(255,255,255,0.05);
    padding: 8px 12px;
    display: flex;
    gap: 6px;
    align-items: center;
  }
  .dot { width: 12px; height: 12px; border-radius: 50%; }
  .dot-red { background: #ff5f56; }
  .dot-yellow { background: #ffbd2e; }
  .dot-green { background: #27c93f; }
  .content {
    padding: 16px;
    white-space: pre-wrap;
    word-wrap: break-word;
    max-width: ${cols}ch;
  }
</style>
</head>
<body>
<div class="terminal">
  <div class="titlebar">
    <div class="dot dot-red"></div>
    <div class="dot dot-yellow"></div>
    <div class="dot dot-green"></div>
  </div>
  <div class="content">${htmlContent}</div>
</div>
</body>
</html>`;
}

export async function captureTerminal(
  scene: TerminalScene,
  config: ResolvedConfig,
): Promise<CaptureResult> {
  const sceneName = scene.name ?? scene.command.replace(/[^a-zA-Z0-9]/g, "_");
  const formats = scene.formats ?? config.output.formats;
  const theme = THEMES[scene.theme ?? "dracula"] ?? THEMES.dracula;
  const maxLines = scene.maxLines ?? 50;
  const cols = scene.cols ?? 80;
  const files: string[] = [];

  log.debug(`Running command: ${scene.command}`);
  const output = await runCommand(scene.command, cols, maxLines);

  const html = buildTerminalHtml(output, theme, cols);

  // Write temp HTML
  const tmpHtml = path.join(os.tmpdir(), `tease-term-${Date.now()}.html`);
  fs.writeFileSync(tmpHtml, html);

  let browser;
  try {
    browser = await chromium.launch();
    const page = await browser.newPage();
    await page.goto(`file://${tmpHtml}`, { waitUntil: "load" });

    // Size the viewport to content
    const termEl = page.locator(".terminal");
    const box = await termEl.boundingBox();
    if (box) {
      await page.setViewportSize({
        width: Math.ceil(box.width + 32),
        height: Math.ceil(box.height + 32),
      });
    }

    if (formats.includes("png")) {
      const pngPath = path.join(config.output.dir, `${sceneName}.png`);
      await page.locator(".terminal").screenshot({ path: pngPath });
      files.push(pngPath);
      log.file(pngPath);
    }

    // For video formats, record the static terminal
    const needsVideo = formats.includes("mp4") || formats.includes("gif");
    if (needsVideo) {
      await page.close();

      // Re-open with video recording
      const context = await browser.newContext({
        recordVideo: {
          dir: path.join(config.output.dir, "_tmp_video"),
          size: {
            width: box ? Math.ceil(box.width + 32) : 800,
            height: box ? Math.ceil(box.height + 32) : 600,
          },
        },
      });
      const videoPage = await context.newPage();
      await videoPage.goto(`file://${tmpHtml}`, { waitUntil: "load" });
      await new Promise((r) => setTimeout(r, config.output.videoDuration));
      await videoPage.close();

      const video = videoPage.video();
      if (video) {
        const webmPath = await video.path();
        if (webmPath) {
          files.push(webmPath);
        }
      }
      await context.close();
    } else {
      await page.close();
    }
  } finally {
    await browser?.close();
    fs.unlinkSync(tmpHtml);
  }

  return { scene, files };
}
