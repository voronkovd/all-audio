import type { Messages } from "../types";

export const en: Messages = {
  app: {
    title: "All Audio",
    subtitle: "Convert audio files between popular formats using FFmpeg.",
  },
  language: {
    label: "Language",
    en: "English",
    ru: "Русский",
    de: "Deutsch",
    fr: "Français",
  },
  input: {
    title: "Input file",
    chooseFile: "Choose file",
    noFile: "No file selected",
    audioFilter: "Audio",
  },
  probe: {
    title: "Source info",
    duration: "Duration",
    codec: "Codec",
    sampleRate: "Sample rate",
    channels: "Channels",
    bitrate: "Bitrate",
    empty: "—",
  },
  output: {
    title: "Output settings",
    format: "Format",
    mp3Bitrate: "MP3 Bitrate",
    saveTo: "Save to",
    noFolder: "No folder selected",
    chooseFolder: "Choose folder",
    filename: "Output filename",
    filenamePlaceholder: "example.mp3",
  },
  cue: {
    title: "CUE Split",
    chooseFile: "Choose CUE file",
    noFile: "No CUE file selected",
    filter: "CUE",
    splitCheckbox: "Split FLAC by CUE",
    warning:
      "CUE splitting is experimental. Please verify track boundaries. If the CUE sheet references multiple FLAC files, select the matching input file.",
  },
  status: {
    label: "Status",
    idle: "Ready",
    checkingFfmpeg: "Checking FFmpeg…",
    probing: "Probing file…",
    converting: "Converting…",
    splitting: "Splitting tracks…",
    done: "Done",
    error: "Error",
    ffmpegBothMissing:
      "Bundled FFmpeg and FFprobe are missing or damaged. Reinstall the app or run npm run fetch-ffmpeg for development builds.",
    ffmpegMissing:
      "Bundled FFmpeg is missing or damaged. Reinstall the app or run npm run fetch-ffmpeg for development builds.",
    ffprobeMissing:
      "Bundled FFprobe is missing or damaged. Reinstall the app or run npm run fetch-ffmpeg for development builds.",
  },
  actions: {
    convert: "Convert",
    splitTracks: "Split tracks",
  },
  success: {
    savedTo: "Saved to {path}",
    createdTracks: "Created {count} tracks.",
    createdFiles: "Created {count} files.",
  },
  errors: {
    unknown: "Unknown error",
    commandNotFound: "Bundled FFmpeg tool could not be executed",
    commandFailed: "Failed to run external command",
    ffprobeFailed: "FFprobe failed to analyze the input file",
    ffprobeParseFailed: "Failed to parse FFprobe output",
    sameInputOutput: "Input and output files must be different",
    cueRequiresFlac: "CUE splitting is only supported for FLAC input files",
    cuePathRequired: "CUE file path is required",
    outputFileExists: "Output file already exists",
    ffmpegFailed: "FFmpeg conversion failed",
    cueReadFailed: "Failed to read CUE file",
    cueMultipleFiles:
      "CUE file references multiple audio files. Select the matching FLAC file for splitting.",
    cueNoTracks: "CUE file does not contain any tracks with INDEX 01",
    invalidInputPath: "Invalid input file path",
    inputNotFound: "Input file does not exist",
    unsupportedInputExtension: "Unsupported input file extension",
    unsupportedOutputFormat: "Unsupported output format",
    invalidOutputDir: "Invalid output directory",
    outputDirNotFound: "Output directory does not exist",
    invalidOutputFilename: "Invalid output filename",
    outputExtensionMismatch:
      "Output filename extension does not match selected format",
    invalidCuePath: "Invalid CUE file path",
    cueNotFound: "CUE file does not exist",
    invalidCueExtension: "CUE file must have a .cue extension",
    invalidMp3Bitrate: "Unsupported MP3 bitrate",
    conversionTaskFailed: "Conversion task failed",
    cueFileMismatch:
      'CUE file does not contain tracks for "{name}". Referenced files: {files}',
    cueAmbiguousFile: 'CUE file contains multiple sections for "{name}"',
    cueInvalidTrackOrder:
      "Track {trackB} starts at the same time or before track {trackA}. Check INDEX 01 values in the CUE file.",
    cueInvalidTrackDuration:
      "Track {trackA} has an invalid duration relative to track {trackB}",
    bundledFfmpegMissing: "Bundled FFmpeg was not found in the application package",
    bundledFfprobeMissing: "Bundled FFprobe was not found in the application package",
    bundledFfmpegNotExecutable: "Bundled FFmpeg binary is not executable",
    bundledFfmpegInvalid: "Bundled FFmpeg binary is invalid",
    bundledFfmpegResolveFailed: "Failed to locate bundled FFmpeg resources",
  },
};
