import { exec } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { promisify } from "node:util";
import ffmpeg from "fluent-ffmpeg";
import { path as ffmpegPath } from "@ffmpeg-installer/ffmpeg";
import { path as ffprobePath } from "@ffprobe-installer/ffprobe";
import screenshot from "screenshot-desktop";
import { log } from "../logger.js";
import type { CaptureResult, ResolvedConfig, ScreenScene } from "../types.js";

const execAsync = promisify(exec);

ffmpeg.setFfmpegPath(ffmpegPath);
ffmpeg.setFfprobePath(ffprobePath);

function getScreenInput(): { format: string; input: string } {
  switch (process.platform) {
    case "darwin":
      return { format: "avfoundation", input: "1:none" };
    case "linux":
      return {
        format: "x11grab",
        input: process.env.DISPLAY ?? ":0.0",
      };
    case "win32":
      return { format: "gdigrab", input: "desktop" };
    default:
      throw new Error(`Unsupported platform: ${process.platform}`);
  }
}

function recordScreen(
  outputPath: string,
  duration: number,
  region?: { x: number; y: number; width: number; height: number },
): Promise<string> {
  return new Promise((resolve, reject) => {
    const { format, input } = getScreenInput();

    let cmd = ffmpeg()
      .input(input)
      .inputFormat(format)
      .duration(duration / 1000);

    if (region) {
      cmd = cmd.videoFilter(
        `crop=${region.width}:${region.height}:${region.x}:${region.y}`,
      );
    }

    cmd
      .outputOptions(["-pix_fmt", "yuv420p"])
      .output(outputPath)
      .on("end", () => resolve(outputPath))
      .on("error", (err) => reject(err))
      .run();
  });
}

export async function captureScreen(
  scene: ScreenScene,
  config: ResolvedConfig,
): Promise<CaptureResult> {
  const sceneName = scene.name ?? `screen_${scene.display ?? 0}`;
  const formats = scene.formats ?? config.output.formats;
  const files: string[] = [];

  // Run setup command if provided
  if (scene.setup) {
    log.debug(`Running setup: ${scene.setup}`);
    await execAsync(scene.setup);
  }

  // Wait for delay
  if (scene.delay) {
    log.debug(`Waiting ${scene.delay}ms...`);
    await new Promise((r) => setTimeout(r, scene.delay));
  }

  // PNG screenshot
  if (formats.includes("png")) {
    const pngPath = path.join(config.output.dir, `${sceneName}.png`);

    const imgBuffer = await screenshot({
      screen: scene.display,
    });

    fs.writeFileSync(pngPath, imgBuffer);
    files.push(pngPath);
    log.file(pngPath);
  }

  // Video recording for mp4/gif
  const needsVideo = formats.includes("mp4") || formats.includes("gif");
  if (needsVideo) {
    const rawPath = path.join(config.output.dir, `${sceneName}_raw.mp4`);
    await recordScreen(rawPath, config.output.videoDuration, scene.region);
    files.push(rawPath);
    log.file(rawPath);
  }

  return { scene, files };
}
