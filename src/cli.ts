import cac from "cac";
import { loadConfig } from "./config.js";
import { log, setVerbose } from "./logger.js";
import { orchestrate } from "./orchestrator.js";

const cli = cac("tease");

cli
  .command("[config]", "Capture project showcase assets")
  .option("-c, --config <path>", "Path to config file")
  .option("-o, --output <dir>", "Output directory")
  .option("--formats <formats>", "Output formats (comma-separated: png,gif,mp4)")
  .option("--no-ai", "Disable Ollama AI mode")
  .option("--verbose", "Enable verbose logging")
  .action(async (_, options) => {
    try {
      if (options.verbose) {
        setVerbose(true);
      }

      log.info("Loading configuration...");
      const config = await loadConfig(options.config);

      if (options.output) {
        config.output.dir = options.output;
      }

      if (options.formats) {
        config.output.formats = options.formats.split(",").map((f: string) => f.trim());
      }

      if (options.ai === false) {
        delete config.ollama;
      }

      const results = await orchestrate(config);

      console.log();
      log.success(`Done! Captured ${results.length} scene(s).`);
      for (const result of results) {
        const name =
          result.scene.name ??
          (result.scene.type === "web"
            ? result.scene.url
            : result.scene.type === "terminal"
              ? result.scene.command
              : "screen");
        log.info(`${name}: ${result.files.length} file(s)`);
        for (const file of result.files) {
          log.file(file);
        }
      }
    } catch (err) {
      log.error(err instanceof Error ? err.message : String(err));
      process.exit(1);
    }
  });

cli.help();
cli.version("0.1.0");
cli.parse();
