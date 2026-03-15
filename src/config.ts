import { cosmiconfig } from "cosmiconfig";
import { TypeScriptLoader } from "cosmiconfig-typescript-loader";
import { z } from "zod";
import type { ResolvedConfig, TeaseConfig } from "./types.js";

const viewportSchema = z.object({
  width: z.number().int().positive(),
  height: z.number().int().positive(),
});

const captureActionSchema = z.object({
  type: z.enum(["scroll-to", "click", "wait", "hover", "screenshot"]),
  selector: z.string().optional(),
  delay: z.number().optional(),
  name: z.string().optional(),
});

const outputFormatSchema = z.enum(["png", "gif", "mp4"]);

const webSceneSchema = z.object({
  type: z.literal("web"),
  url: z.string(),
  name: z.string().optional(),
  viewport: viewportSchema.optional(),
  formats: z.array(outputFormatSchema).optional(),
  actions: z.array(captureActionSchema).optional(),
});

const screenSceneSchema = z.object({
  type: z.literal("screen"),
  name: z.string().optional(),
  display: z.number().int().nonnegative().optional(),
  region: z
    .object({
      x: z.number(),
      y: z.number(),
      width: z.number().positive(),
      height: z.number().positive(),
    })
    .optional(),
  formats: z.array(outputFormatSchema).optional(),
  setup: z.string().optional(),
  delay: z.number().optional(),
});

const terminalSceneSchema = z.object({
  type: z.literal("terminal"),
  command: z.string(),
  name: z.string().optional(),
  formats: z.array(outputFormatSchema).optional(),
  theme: z.string().optional(),
  maxLines: z.number().int().positive().optional(),
  cols: z.number().int().positive().optional(),
});

const sceneSchema = z.discriminatedUnion("type", [
  webSceneSchema,
  screenSceneSchema,
  terminalSceneSchema,
]);

const configSchema = z.object({
  scenes: z.array(sceneSchema).min(1),
  server: z
    .object({
      command: z.string(),
      url: z.string().url(),
      timeout: z.number().positive().optional(),
    })
    .optional(),
  viewport: viewportSchema.optional(),
  output: z
    .object({
      dir: z.string().optional(),
      formats: z.array(outputFormatSchema).optional(),
      videoDuration: z.number().positive().optional(),
    })
    .optional(),
  ollama: z
    .object({
      endpoint: z.string().optional(),
      model: z.string().optional(),
      prompt: z.string().optional(),
    })
    .optional(),
});

const explorer = cosmiconfig("tease", {
  loaders: {
    ".ts": TypeScriptLoader(),
  },
});

export async function loadConfig(
  configPath?: string,
): Promise<ResolvedConfig> {
  const result = configPath
    ? await explorer.load(configPath)
    : await explorer.search();

  if (!result || result.isEmpty) {
    throw new Error(
      "No configuration found. Create a tease.config.ts file or pass --config.",
    );
  }

  const raw = result.config?.default ?? result.config;
  const parsed = configSchema.parse(raw) as TeaseConfig;

  return resolveConfig(parsed);
}

export function resolveConfig(config: TeaseConfig): ResolvedConfig {
  return {
    scenes: config.scenes,
    server: config.server,
    viewport: config.viewport ?? { width: 1280, height: 720 },
    output: {
      dir: config.output?.dir ?? "./tease-output",
      formats: config.output?.formats ?? ["png"],
      videoDuration: config.output?.videoDuration ?? 5000,
    },
    ollama: config.ollama,
  };
}
