export const OUTPUT_FORMATS = [
  "mp3",
  "wav",
  "flac",
  "ogg",
  "opus",
  "m4a",
  "aac",
] as const;

export const MP3_BITRATES = ["128k", "192k", "256k", "320k"] as const;

export type OutputFormat = (typeof OUTPUT_FORMATS)[number];
export type Mp3Bitrate = (typeof MP3_BITRATES)[number];

export type AppStatus =
  | "idle"
  | "checking_ffmpeg"
  | "probing"
  | "converting"
  | "splitting"
  | "done"
  | "error";

export interface FfmpegAvailability {
  ffmpeg: boolean;
  ffprobe: boolean;
}

export interface AudioProbe {
  duration: number | null;
  codec: string | null;
  sample_rate: number | null;
  channels: number | null;
  bitrate: number | null;
}

export interface ConvertAudioResult {
  output_paths: string[];
}

export interface CommandError {
  code: string;
  message: string;
}

export interface ConvertAudioOptions {
  mp3Bitrate?: Mp3Bitrate;
  cuePath?: string;
  splitByCue?: boolean;
}
