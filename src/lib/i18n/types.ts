export type Locale = "en" | "ru" | "de" | "fr";

export interface Messages {
  app: {
    title: string;
    subtitle: string;
  };
  language: {
    label: string;
    en: string;
    ru: string;
    de: string;
    fr: string;
  };
  input: {
    title: string;
    chooseFile: string;
    noFile: string;
    audioFilter: string;
  };
  probe: {
    title: string;
    duration: string;
    codec: string;
    sampleRate: string;
    channels: string;
    bitrate: string;
    empty: string;
  };
  output: {
    title: string;
    format: string;
    mp3Bitrate: string;
    saveTo: string;
    noFolder: string;
    chooseFolder: string;
    filename: string;
    filenamePlaceholder: string;
  };
  cue: {
    title: string;
    chooseFile: string;
    noFile: string;
    filter: string;
    splitCheckbox: string;
    warning: string;
  };
  mp3Tags: {
    title: string;
    hint: string;
    cueHint: string;
    titleField: string;
    titlePlaceholder: string;
    artist: string;
    artistPlaceholder: string;
    album: string;
    albumPlaceholder: string;
    albumArtist: string;
    albumArtistPlaceholder: string;
    year: string;
    yearPlaceholder: string;
    genre: string;
    genrePlaceholder: string;
    track: string;
    trackPlaceholder: string;
    cover: string;
    noCover: string;
    chooseCover: string;
    clearCover: string;
    coverFilter: string;
  };
  status: {
    label: string;
    idle: string;
    checkingFfmpeg: string;
    probing: string;
    converting: string;
    splitting: string;
    done: string;
    error: string;
    ffmpegBothMissing: string;
    ffmpegMissing: string;
    ffprobeMissing: string;
  };
  actions: {
    convert: string;
    splitTracks: string;
  };
  success: {
    savedTo: string;
    createdTracks: string;
    createdFiles: string;
  };
  errors: {
    unknown: string;
    commandNotFound: string;
    commandFailed: string;
    ffprobeFailed: string;
    ffprobeParseFailed: string;
    sameInputOutput: string;
    cueRequiresFlac: string;
    cuePathRequired: string;
    outputFileExists: string;
    ffmpegFailed: string;
    cueReadFailed: string;
    cueMultipleFiles: string;
    cueNoTracks: string;
    invalidInputPath: string;
    inputNotFound: string;
    unsupportedInputExtension: string;
    unsupportedOutputFormat: string;
    invalidOutputDir: string;
    outputDirNotFound: string;
    invalidOutputFilename: string;
    outputExtensionMismatch: string;
    invalidCuePath: string;
    cueNotFound: string;
    invalidCueExtension: string;
    invalidMp3Bitrate: string;
    conversionTaskFailed: string;
    cueFileMismatch: string;
    cueAmbiguousFile: string;
    cueInvalidTrackOrder: string;
    cueInvalidTrackDuration: string;
    bundledFfmpegMissing: string;
    bundledFfprobeMissing: string;
    bundledFfmpegNotExecutable: string;
    bundledFfmpegInvalid: string;
    bundledFfmpegResolveFailed: string;
    invalidCoverPath: string;
    coverNotFound: string;
    unsupportedCoverExtension: string;
  };
}
