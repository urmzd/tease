import pc from "picocolors";

let verbose = false;

export function setVerbose(v: boolean): void {
  verbose = v;
}

export const log = {
  info(msg: string): void {
    console.log(`${pc.blue("ℹ")} ${msg}`);
  },
  success(msg: string): void {
    console.log(`${pc.green("✔")} ${msg}`);
  },
  warn(msg: string): void {
    console.log(`${pc.yellow("⚠")} ${msg}`);
  },
  error(msg: string): void {
    console.error(`${pc.red("✖")} ${msg}`);
  },
  debug(msg: string): void {
    if (verbose) {
      console.log(`${pc.gray("●")} ${msg}`);
    }
  },
  scene(name: string, type: string): void {
    console.log(`\n${pc.cyan("▶")} ${pc.bold(name)} ${pc.gray(`[${type}]`)}`);
  },
  file(path: string): void {
    console.log(`  ${pc.green("→")} ${path}`);
  },
};
