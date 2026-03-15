import path from "node:path";
import ffmpeg from "fluent-ffmpeg";
import { path as ffmpegPath } from "@ffmpeg-installer/ffmpeg";
import { path as ffprobePath } from "@ffprobe-installer/ffprobe";
import { log } from "./logger.js";

ffmpeg.setFfmpegPath(ffmpegPath);
ffmpeg.setFfprobePath(ffprobePath);

export function convertToMp4(inputPath: string, outputPath: string): Promise<string> {
  return new Promise((resolve, reject) => {
    log.debug(`Converting to MP4: ${inputPath} → ${outputPath}`);
    ffmpeg(inputPath)
      .videoCodec("libx264")
      .outputOptions(["-pix_fmt", "yuv420p", "-movflags", "+faststart"])
      .output(outputPath)
      .on("end", () => {
        log.file(outputPath);
        resolve(outputPath);
      })
      .on("error", reject)
      .run();
  });
}

export function convertToGif(inputPath: string, outputPath: string): Promise<string> {
  return new Promise((resolve, reject) => {
    log.debug(`Converting to GIF: ${inputPath} → ${outputPath}`);
    ffmpeg(inputPath)
      .outputOptions([
        "-vf",
        "fps=15,scale=800:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
      ])
      .output(outputPath)
      .on("end", () => {
        log.file(outputPath);
        resolve(outputPath);
      })
      .on("error", reject)
      .run();
  });
}

export async function convertVideoFiles(
  videoPath: string,
  baseName: string,
  outputDir: string,
  formats: string[],
): Promise<string[]> {
  const results: string[] = [];

  if (formats.includes("mp4")) {
    const mp4Path = path.join(outputDir, `${baseName}.mp4`);
    await convertToMp4(videoPath, mp4Path);
    results.push(mp4Path);
  }

  if (formats.includes("gif")) {
    const gifPath = path.join(outputDir, `${baseName}.gif`);
    await convertToGif(videoPath, gifPath);
    results.push(gifPath);
  }

  return results;
}
