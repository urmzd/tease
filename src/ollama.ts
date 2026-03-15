import fs from "node:fs";
import { log } from "./logger.js";
import type { OllamaConfig } from "./types.js";

const DEFAULT_ENDPOINT = "http://localhost:11434";
const DEFAULT_MODEL = "llama3.2-vision";

export async function isOllamaAvailable(config: OllamaConfig): Promise<boolean> {
  const endpoint = config.endpoint ?? DEFAULT_ENDPOINT;
  try {
    const response = await fetch(`${endpoint}/api/tags`);
    return response.ok;
  } catch {
    return false;
  }
}

export async function analyzeScreenshot(
  imagePath: string,
  config: OllamaConfig,
): Promise<string | undefined> {
  const endpoint = config.endpoint ?? DEFAULT_ENDPOINT;
  const model = config.model ?? DEFAULT_MODEL;
  const prompt =
    config.prompt ??
    "Analyze this screenshot of a web application. Suggest what areas would be most interesting to capture for a project showcase. Be concise — respond with a short list of suggestions.";

  try {
    const imageBuffer = fs.readFileSync(imagePath);
    const base64Image = imageBuffer.toString("base64");

    const response = await fetch(`${endpoint}/api/generate`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        model,
        prompt,
        images: [base64Image],
        stream: false,
      }),
    });

    if (!response.ok) {
      log.warn(`Ollama returned ${response.status}`);
      return undefined;
    }

    const data = (await response.json()) as { response: string };
    return data.response;
  } catch (err) {
    log.warn(`Ollama error: ${err instanceof Error ? err.message : String(err)}`);
    return undefined;
  }
}
