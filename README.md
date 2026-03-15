# tease

Automated project showcase capture -- screenshots, GIFs, and videos from web apps, desktop, and terminal.

## Features

- **Web capture** -- navigate to URLs via Playwright, run actions (click, scroll, hover), and capture the results.
- **Screen capture** -- grab your desktop or a specific display/region, with optional setup commands and delay.
- **Terminal capture** -- record CLI output from any command, with configurable themes, columns, and line limits.
- **AI mode** -- send captures to a local Ollama vision model for automated analysis and suggestions.

## Quick Start

```bash
npm install @urmzd/tease
```

Create a `tease.config.ts` in your project root:

```ts
import { defineConfig } from "@urmzd/tease";

export default defineConfig({
  server: { command: "npm run dev", url: "http://localhost:3000" },
  scenes: [
    { type: "web", url: "/", name: "homepage" },
    { type: "screen", name: "native-app", setup: "open MyApp.app", delay: 3000 },
    { type: "terminal", command: "npm test", name: "tests", theme: "dracula" },
  ],
  output: { dir: "./showcase", formats: ["png", "mp4"] },
});
```

Run it:

```bash
npx @urmzd/tease
```

## Configuration

Each entry in `scenes` is one of three types:

| Type       | Key fields                                      |
|------------|--------------------------------------------------|
| `web`      | `url`, `viewport`, `actions`, `formats`          |
| `screen`   | `display`, `region`, `setup`, `delay`, `formats` |
| `terminal` | `command`, `theme`, `cols`, `maxLines`, `formats` |

All scenes accept `name` and `formats`. The top-level `output.formats` serves as the default.

A `server` block (optional) starts a dev server before capture and waits for it to be ready.

## CLI Usage

```
tease [config]

Options:
  -c, --config <path>      Path to config file
  -o, --output <dir>       Output directory
  --formats <formats>      Output formats (comma-separated: png,gif,mp4)
  --no-ai                  Disable Ollama AI mode
  --verbose                Enable verbose logging
  -h, --help               Display help
  -v, --version            Display version
```

## Output Formats

| Format | Description              |
|--------|--------------------------|
| `png`  | Static screenshot        |
| `gif`  | Animated capture         |
| `mp4`  | Full video recording     |

Set formats globally via `output.formats` or per-scene via `formats`.

## GitHub Action

```yaml
- uses: ./.github/actions/tease
  with:
    formats: "png,gif"
    output: "./showcase"
    ollama-model: "llama3.2-vision"  # omit to skip AI
```

Captured assets are automatically uploaded as build artifacts.

## Ollama AI Mode

When an `ollama` block is present in your config, tease sends each capture to a local Ollama vision model for analysis.

```ts
export default defineConfig({
  scenes: [/* ... */],
  ollama: {
    model: "llama3.2-vision",
    endpoint: "http://localhost:11434", // default
    prompt: "Describe this UI screenshot.",
  },
});
```

Disable at runtime with `--no-ai`.

## License

MIT
