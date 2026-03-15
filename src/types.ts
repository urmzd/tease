export interface ViewportConfig {
  width: number;
  height: number;
}

export type OutputFormat = "png" | "gif" | "mp4";

export interface CaptureAction {
  type: "scroll-to" | "click" | "wait" | "hover" | "screenshot";
  selector?: string;
  delay?: number;
  name?: string;
}

export interface WebScene {
  type: "web";
  url: string;
  name?: string;
  viewport?: ViewportConfig;
  formats?: OutputFormat[];
  actions?: CaptureAction[];
}

export interface ScreenScene {
  type: "screen";
  name?: string;
  display?: number;
  region?: { x: number; y: number; width: number; height: number };
  formats?: OutputFormat[];
  setup?: string;
  delay?: number;
}

export interface TerminalScene {
  type: "terminal";
  command: string;
  name?: string;
  formats?: OutputFormat[];
  theme?: string;
  maxLines?: number;
  cols?: number;
}

export type SceneConfig = WebScene | ScreenScene | TerminalScene;

export interface ServerConfig {
  command: string;
  url: string;
  timeout?: number;
}

export interface OutputConfig {
  dir?: string;
  formats?: OutputFormat[];
  videoDuration?: number;
}

export interface OllamaConfig {
  endpoint?: string;
  model?: string;
  prompt?: string;
}

export interface TeaseConfig {
  scenes: SceneConfig[];
  server?: ServerConfig;
  viewport?: ViewportConfig;
  output?: OutputConfig;
  ollama?: OllamaConfig;
}

export interface ResolvedConfig {
  scenes: SceneConfig[];
  server?: ServerConfig;
  viewport: ViewportConfig;
  output: {
    dir: string;
    formats: OutputFormat[];
    videoDuration: number;
  };
  ollama?: OllamaConfig;
}

export interface CaptureResult {
  scene: SceneConfig;
  files: string[];
  aiSuggestions?: string;
}
