import type { Messages } from "./types";
import { locales } from "./locales";
import type { Locale } from "./types";

const ERROR_CODE_MAP: Record<string, keyof Messages["errors"]> = {
  command_not_found: "commandNotFound",
  command_failed: "commandFailed",
  ffprobe_failed: "ffprobeFailed",
  ffprobe_parse_failed: "ffprobeParseFailed",
  same_input_output: "sameInputOutput",
  cue_requires_flac: "cueRequiresFlac",
  cue_path_required: "cuePathRequired",
  output_file_exists: "outputFileExists",
  ffmpeg_failed: "ffmpegFailed",
  cue_read_failed: "cueReadFailed",
  cue_multiple_files: "cueMultipleFiles",
  cue_no_tracks: "cueNoTracks",
  invalid_input_path: "invalidInputPath",
  input_not_found: "inputNotFound",
  unsupported_input_extension: "unsupportedInputExtension",
  unsupported_output_format: "unsupportedOutputFormat",
  invalid_output_dir: "invalidOutputDir",
  output_dir_not_found: "outputDirNotFound",
  invalid_output_filename: "invalidOutputFilename",
  output_extension_mismatch: "outputExtensionMismatch",
  invalid_cue_path: "invalidCuePath",
  cue_not_found: "cueNotFound",
  invalid_cue_extension: "invalidCueExtension",
  invalid_mp3_bitrate: "invalidMp3Bitrate",
  conversion_task_failed: "conversionTaskFailed",
  cue_file_mismatch: "cueFileMismatch",
  cue_ambiguous_file: "cueAmbiguousFile",
  cue_invalid_track_order: "cueInvalidTrackOrder",
  cue_invalid_track_duration: "cueInvalidTrackDuration",
  bundled_ffmpeg_missing: "bundledFfmpegMissing",
  bundled_ffprobe_missing: "bundledFfprobeMissing",
  bundled_ffmpeg_not_executable: "bundledFfmpegNotExecutable",
  bundled_ffmpeg_invalid: "bundledFfmpegInvalid",
  bundled_ffmpeg_resolve_failed: "bundledFfmpegResolveFailed",
  invalid_cover_path: "invalidCoverPath",
  cover_not_found: "coverNotFound",
  unsupported_cover_extension: "unsupportedCoverExtension",
};

function interpolate(
  template: string,
  params: Record<string, string | number>,
): string {
  return Object.entries(params).reduce(
    (result, [key, value]) => result.replaceAll(`{${key}}`, String(value)),
    template,
  );
}

function extractTrackNumbers(
  message: string,
  kind: "order" | "duration",
): { trackA?: string; trackB?: string } {
  if (kind === "order") {
    const match = message.match(/Track (\d+).*before track (\d+)/i);
    if (match) {
      return { trackB: match[1], trackA: match[2] };
    }
  }

  const match = message.match(/Track (\d+).*relative to track (\d+)/i);
  if (match) {
    return { trackA: match[1], trackB: match[2] };
  }

  return {};
}

function extractCueFileMismatch(
  message: string,
): { name?: string; files?: string } {
  const match = message.match(/for "([^"]+)".*Referenced files: (.+)$/i);
  if (!match) {
    return {};
  }

  return { name: match[1], files: match[2] };
}

function extractCueAmbiguous(message: string): { name?: string } {
  const match = message.match(/for "([^"]+)"/i);
  return match ? { name: match[1] } : {};
}

export function translateError(
  error: unknown,
  locale: Locale,
): string {
  const messages = locales[locale].errors;

  if (typeof error === "string") {
    return error;
  }

  if (!error || typeof error !== "object") {
    return messages.unknown;
  }

  const code =
    "code" in error && typeof error.code === "string" ? error.code : undefined;
  const message =
    "message" in error && typeof error.message === "string"
      ? error.message
      : messages.unknown;

  if (!code) {
    return message;
  }

  const key = ERROR_CODE_MAP[code];
  if (!key) {
    return message;
  }

  const template = messages[key];

  if (code === "cue_file_mismatch") {
    const params = extractCueFileMismatch(message);
    return interpolate(template, {
      name: params.name ?? "",
      files: params.files ?? "",
    });
  }

  if (code === "cue_ambiguous_file") {
    const params = extractCueAmbiguous(message);
    return interpolate(template, { name: params.name ?? "" });
  }

  if (code === "cue_invalid_track_order") {
    const params = extractTrackNumbers(message, "order");
    return interpolate(template, {
      trackA: params.trackA ?? "?",
      trackB: params.trackB ?? "?",
    });
  }

  if (code === "cue_invalid_track_duration") {
    const params = extractTrackNumbers(message, "duration");
    return interpolate(template, {
      trackA: params.trackA ?? "?",
      trackB: params.trackB ?? "?",
    });
  }

  if (code === "output_file_exists" && message.includes(":")) {
    return message;
  }

  if (code === "ffmpeg_failed" || code === "ffprobe_failed") {
    const stderr = message.trim();
    return stderr.length > 0 ? stderr : template;
  }

  return template;
}
