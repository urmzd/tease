import fs from "node:fs";
import path from "node:path";
import { captureScreen } from "./capture/screen.js";
import { captureTerminal } from "./capture/terminal.js";
import { captureWeb } from "./capture/web.js";
import { convertVideoFiles } from "./convert.js";
import { log } from "./logger.js";
import { analyzeScreenshot, isOllamaAvailable } from "./ollama.js";
import { startServer } from "./server.js";
import type { CaptureResult, ResolvedConfig, SceneConfig } from "./types.js";

function getSceneName(scene: SceneConfig): string {
  if (scene.name) return scene.name;
  switch (scene.type) {
    case "web":
      return scene.url.replace(/[^a-zA-Z0-9]/g, "_");
    case "terminal":
      return scene.command.replace(/[^a-zA-Z0-9]/g, "_");
    case "screen":
      return `screen_${scene.display ?? 0}`;
  }
}

async function processVideoConversions(
  result: CaptureResult,
  config: ResolvedConfig,
): Promise<string[]> {
  const formats = ("formats" in result.scene && result.scene.formats) || config.output.formats;
  const sceneName = getSceneName(result.scene);
  const convertedFiles: string[] = [];

  // Find video files (webm or raw mp4) that need conversion
  for (const file of result.files) {
    const ext = path.extname(file).toLowerCase();
    if (ext === ".webm" || file.includes("_raw.mp4")) {
      const converted = await convertVideoFiles(
        file,
        sceneName,
        config.output.dir,
        formats,
      );
      convertedFiles.push(...converted);

      // Clean up source video
      try {
        fs.unlinkSync(file);
      } catch {
        // ignore
      }
    }
  }

  return convertedFiles;
}

export async function orchestrate(
  config: ResolvedConfig,
): Promise<CaptureResult[]> {
  // Ensure output directory exists
  fs.mkdirSync(config.output.dir, { recursive: true });
  fs.mkdirSync(path.join(config.output.dir, "_tmp_video"), { recursive: true });

  // Check Ollama availability
  let aiEnabled = false;
  if (config.ollama) {
    aiEnabled = await isOllamaAvailable(config.ollama);
    if (aiEnabled) {
      log.success("Ollama is available — AI mode enabled.");
    } else {
      log.warn("Ollama not available — falling back to standard capture.");
    }
  }

  // Start dev server if needed
  const hasWebScenes = config.scenes.some((s) => s.type === "web");
  let stopServer: (() => void) | undefined;

  if (config.server && hasWebScenes) {
    stopServer = await startServer(config.server);
  }

  const results: CaptureResult[] = [];

  try {
    // Process scenes sequentially to avoid OOM
    for (const scene of config.scenes) {
      const name = getSceneName(scene);
      log.scene(name, scene.type);

      let result: CaptureResult;

      switch (scene.type) {
        case "web":
          result = await captureWeb(scene, config);
          break;
        case "screen":
          result = await captureScreen(scene, config);
          break;
        case "terminal":
          result = await captureTerminal(scene, config);
          break;
      }

      // AI analysis
      if (aiEnabled && config.ollama) {
        const pngFile = result.files.find((f) => f.endsWith(".png"));
        if (pngFile) {
          log.debug("Sending screenshot to Ollama for analysis...");
          const suggestions = await analyzeScreenshot(pngFile, config.ollama);
          if (suggestions) {
            result.aiSuggestions = suggestions;
            log.info(`AI suggestions: ${suggestions.slice(0, 100)}...`);
          }
        }
      }

      // Convert video files
      const convertedFiles = await processVideoConversions(result, config);
      result.files = [
        ...result.files.filter(
          (f) => !f.endsWith(".webm") && !f.includes("_raw.mp4"),
        ),
        ...convertedFiles,
      ];

      results.push(result);
    }
  } finally {
    stopServer?.();

    // Clean up temp video directory
    const tmpVideoDir = path.join(config.output.dir, "_tmp_video");
    try {
      fs.rmSync(tmpVideoDir, { recursive: true, force: true });
    } catch {
      // ignore
    }
  }

  return results;
}
