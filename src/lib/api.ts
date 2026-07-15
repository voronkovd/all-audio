import { translateError } from "$lib/i18n/errors";
import { getLocale } from "$lib/i18n/store";

import { invoke } from "@tauri-apps/api/core";

import type {
  AudioProbe,
  ConvertAudioOptions,
  ConvertAudioResult,
  CueAlbumInfo,
  FfmpegAvailability,
  OutputFormat,
} from "./types";

export function parseCommandError(error: unknown): string {
  return translateError(error, getLocale());
}

export async function checkFfmpegAvailable(): Promise<FfmpegAvailability> {
  return invoke<FfmpegAvailability>("check_ffmpeg_available");
}

export async function probeAudioFile(inputPath: string): Promise<AudioProbe> {
  return invoke<AudioProbe>("probe_audio_file", { inputPath });
}

export async function readCueAlbumInfo(cuePath: string): Promise<CueAlbumInfo> {
  return invoke<CueAlbumInfo>("read_cue_album_info", { cuePath });
}

export async function convertAudio(
  inputPath: string,
  outputDir: string,
  outputFilename: string,
  outputFormat: OutputFormat,
  options: ConvertAudioOptions = {},
): Promise<ConvertAudioResult> {
  return invoke<ConvertAudioResult>("convert_audio", {
    request: {
      input_path: inputPath,
      output_dir: outputDir,
      output_filename: outputFilename,
      output_format: outputFormat,
      mp3_bitrate: options.mp3Bitrate,
      cue_path: options.cuePath,
      split_by_cue: options.splitByCue,
      mp3_tags: options.mp3Tags,
      cover_path: options.coverPath,
    },
  });
}
