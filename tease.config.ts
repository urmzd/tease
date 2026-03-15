import { defineConfig } from "./src/index.js";

export default defineConfig({
  server: { command: "npm run dev", url: "http://localhost:3000" },
  scenes: [
    // Web pages
    { type: "web", url: "/", name: "homepage" },
    { type: "web", url: "/dashboard", name: "dashboard", formats: ["png", "gif"] },

    // Desktop app
    { type: "screen", name: "native-app", setup: "open MyApp.app", delay: 3000 },

    // CLI output
    { type: "terminal", command: "npm test", name: "tests", theme: "dracula" },
    { type: "terminal", command: "my-cli --help", name: "cli-help" },
  ],
  output: { dir: "./showcase", formats: ["png", "mp4"] },
  ollama: { model: "llama3.2-vision" },
});
