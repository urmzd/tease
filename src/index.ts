import type { TeaseConfig } from "./types.js";

export function defineConfig(config: TeaseConfig): TeaseConfig {
  return config;
}

export type {
  TeaseConfig,
  SceneConfig,
  WebScene,
  ScreenScene,
  TerminalScene,
  ViewportConfig,
  OutputFormat,
  CaptureAction,
  ServerConfig,
  OutputConfig,
  OllamaConfig,
  ResolvedConfig,
  CaptureResult,
} from "./types.js";
