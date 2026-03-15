import path from "node:path";
import { chromium, type Browser, type Page } from "playwright";
import { log } from "../logger.js";
import type {
  CaptureAction,
  CaptureResult,
  OutputFormat,
  ResolvedConfig,
  WebScene,
} from "../types.js";

async function executeAction(page: Page, action: CaptureAction): Promise<void> {
  switch (action.type) {
    case "click":
      if (action.selector) {
        await page.click(action.selector);
      }
      break;
    case "hover":
      if (action.selector) {
        await page.hover(action.selector);
      }
      break;
    case "scroll-to":
      if (action.selector) {
        await page.locator(action.selector).scrollIntoViewIfNeeded();
      }
      break;
    case "wait":
      await new Promise((r) => setTimeout(r, action.delay ?? 1000));
      break;
    case "screenshot":
      // Handled externally — this is a no-op marker
      break;
  }

  if (action.delay && action.type !== "wait") {
    await new Promise((r) => setTimeout(r, action.delay));
  }
}

export async function captureWeb(
  scene: WebScene,
  config: ResolvedConfig,
): Promise<CaptureResult> {
  const sceneName = scene.name ?? scene.url.replace(/[^a-zA-Z0-9]/g, "_");
  const formats = scene.formats ?? config.output.formats;
  const viewport = scene.viewport ?? config.viewport;
  const files: string[] = [];

  const needsVideo = formats.includes("mp4") || formats.includes("gif");

  let browser: Browser | undefined;

  try {
    browser = await chromium.launch();
    const contextOptions: Record<string, unknown> = {
      viewport,
    };

    if (needsVideo) {
      contextOptions.recordVideo = {
        dir: path.join(config.output.dir, "_tmp_video"),
        size: viewport,
      };
    }

    const context = await browser.newContext(contextOptions);
    const page = await context.newPage();

    const fullUrl = scene.url.startsWith("http")
      ? scene.url
      : `${config.server?.url ?? "http://localhost:3000"}${scene.url}`;

    log.debug(`Navigating to ${fullUrl}`);
    await page.goto(fullUrl, { waitUntil: "networkidle" });

    // Execute actions
    if (scene.actions) {
      for (const action of scene.actions) {
        log.debug(`Action: ${action.type} ${action.selector ?? ""}`);
        await executeAction(page, action);
      }
    }

    // PNG screenshot
    if (formats.includes("png")) {
      const pngPath = path.join(config.output.dir, `${sceneName}.png`);
      await page.screenshot({ path: pngPath, fullPage: true });
      files.push(pngPath);
      log.file(pngPath);
    }

    // Video recording
    if (needsVideo) {
      // Keep page open for video duration
      await new Promise((r) => setTimeout(r, config.output.videoDuration));
      await page.close();
      const video = page.video();
      if (video) {
        const webmPath = await video.path();
        if (webmPath) {
          files.push(webmPath);
        }
      }
    } else {
      await page.close();
    }

    await context.close();
  } finally {
    await browser?.close();
  }

  return { scene, files };
}
