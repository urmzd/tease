declare module "screenshot-desktop" {
  interface ScreenshotOptions {
    screen?: number;
    format?: string;
  }
  function screenshot(options?: ScreenshotOptions): Promise<Buffer>;
  export = screenshot;
}
