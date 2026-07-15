<script lang="ts">
  import { onMount } from "svelte";
  import { fromStore } from "svelte/store";

  import AudioProbeInfo from "$lib/components/AudioProbeInfo.svelte";
  import CueSplit from "$lib/components/CueSplit.svelte";
  import InputFilePicker from "$lib/components/InputFilePicker.svelte";
  import LanguageSelector from "$lib/components/LanguageSelector.svelte";
  import Mp3TagsSettings from "$lib/components/Mp3TagsSettings.svelte";
  import OutputSettings from "$lib/components/OutputSettings.svelte";
  import StatusPanel from "$lib/components/StatusPanel.svelte";
  import {
    checkFfmpegAvailable,
    convertAudio,
    parseCommandError,
    probeAudioFile,
    readCueAlbumInfo,
  } from "$lib/api";
  import { isFlacFile } from "$lib/audio";
  import { defaultOutputName } from "$lib/format";
  import { formatMessage, locale, messages } from "$lib/i18n/store";
  import {
    emptyMp3Tags,
    fillEmptyMp3Tags,
    mp3TagsFromMetadata,
    mp3TagsToApi,
  } from "$lib/mp3";
  import type { AppStatus, AudioProbe, Mp3Bitrate, OutputFormat } from "$lib/types";
  import type { Mp3Tags } from "$lib/mp3";

  let inputPath = $state("");
  let outputDir = $state("");
  let outputFilename = $state("");
  let outputFormat = $state<OutputFormat>("mp3");
  let mp3Bitrate = $state<Mp3Bitrate>("320k");
  let mp3Tags = $state<Mp3Tags>(emptyMp3Tags());
  let coverPath = $state("");
  let cuePath = $state("");
  let splitByCue = $state(false);
  let lastError = $state<unknown>(null);
  let successInfo = $state<
    | { kind: "saved"; path: string }
    | { kind: "tracks"; count: number }
    | { kind: "files"; count: number }
    | null
  >(null);
  let probe = $state<AudioProbe | null>(null);
  let status = $state<AppStatus>("checking_ffmpeg");
  let createdFiles = $state<string[]>([]);
  let ffmpegAvailable = $state(false);
  let ffprobeAvailable = $state(false);

  const localeState = fromStore(locale);
  const messagesState = fromStore(messages);

  const errorMessage = $derived.by(() => {
    localeState.current;
    return lastError ? parseCommandError(lastError) : "";
  });

  const successMessage = $derived.by(() => {
    localeState.current;
    const m = messagesState.current;
    if (!successInfo) {
      return "";
    }

    if (successInfo.kind === "saved") {
      return formatMessage(m.success.savedTo, { path: successInfo.path });
    }

    if (successInfo.kind === "tracks") {
      return formatMessage(m.success.createdTracks, { count: successInfo.count });
    }

    return formatMessage(m.success.createdFiles, { count: successInfo.count });
  });

  const isFlacInput = $derived(isFlacFile(inputPath));

  const isBusy = $derived(
    status === "checking_ffmpeg" ||
      status === "probing" ||
      status === "converting" ||
      status === "splitting",
  );

  const canConvert = $derived(
    ffmpegAvailable &&
      ffprobeAvailable &&
      inputPath.length > 0 &&
      outputDir.length > 0 &&
      !isBusy &&
      (splitByCue ? cuePath.length > 0 : outputFilename.trim().length > 0),
  );

  async function refreshFfmpegStatus() {
    status = "checking_ffmpeg";
    lastError = null;
    successInfo = null;
    createdFiles = [];

    try {
      const availability = await checkFfmpegAvailable();
      ffmpegAvailable = availability.ffmpeg;
      ffprobeAvailable = availability.ffprobe;

      if (!ffmpegAvailable || !ffprobeAvailable) {
        status = "error";
        return;
      }

      status = "idle";
    } catch (error) {
      status = "error";
      lastError = error;
    }
  }

  async function applyCueAlbumTags(path: string) {
    try {
      const album = await readCueAlbumInfo(path);
      mp3Tags = fillEmptyMp3Tags(mp3Tags, {
        album: album.title ?? "",
        albumArtist: album.performer ?? "",
        artist: album.performer ?? "",
      });
    } catch {
      // CUE album metadata is optional for the UI.
    }
  }

  async function handleInputSelect(path: string) {
    inputPath = path;
    probe = null;
    lastError = null;
    successInfo = null;
    createdFiles = [];
    outputFilename = defaultOutputName(path, outputFormat);
    mp3Tags = emptyMp3Tags();
    coverPath = "";

    if (!isFlacFile(path)) {
      cuePath = "";
      splitByCue = false;
    }

    if (!ffprobeAvailable) {
      return;
    }

    status = "probing";

    try {
      probe = await probeAudioFile(path);
      mp3Tags = mp3TagsFromMetadata(probe.metadata);
      status = "idle";
    } catch (error) {
      status = "error";
      lastError = error;
    }
  }

  function handleFormatChange(format: OutputFormat) {
    outputFormat = format;

    if (inputPath) {
      outputFilename = defaultOutputName(inputPath, format);
    }
  }

  function handleCueSelect(path: string) {
    cuePath = path;
    lastError = null;
    successInfo = null;
    createdFiles = [];
    void applyCueAlbumTags(path);
  }

  function handleSplitByCueChange(enabled: boolean) {
    splitByCue = enabled;
    lastError = null;
    successInfo = null;
    createdFiles = [];
  }

  async function handleConvert() {
    lastError = null;
    successInfo = null;
    createdFiles = [];
    status = splitByCue ? "splitting" : "converting";

    try {
      const result = await convertAudio(
        inputPath,
        outputDir,
        outputFilename.trim(),
        outputFormat,
        {
          mp3Bitrate: outputFormat === "mp3" ? mp3Bitrate : undefined,
          cuePath: splitByCue ? cuePath : undefined,
          splitByCue,
          mp3Tags: outputFormat === "mp3" ? mp3TagsToApi(mp3Tags) : undefined,
          coverPath:
            outputFormat === "mp3" && coverPath.trim().length > 0
              ? coverPath
              : undefined,
        },
      );

      createdFiles = result.output_paths;

      if (splitByCue) {
        successInfo = { kind: "tracks", count: result.output_paths.length };
      } else if (result.output_paths.length === 1) {
        successInfo = { kind: "saved", path: result.output_paths[0] };
      } else {
        successInfo = { kind: "files", count: result.output_paths.length };
      }

      status = "done";
    } catch (error) {
      status = "error";
      lastError = error;
    }
  }

  onMount(() => {
    void refreshFfmpegStatus();
  });
</script>

<main class="app">
  <header>
    <div class="header-row">
      <div>
        <h1>{$messages.app.title}</h1>
        <p>{$messages.app.subtitle}</p>
      </div>
      <LanguageSelector />
    </div>
  </header>

  <div class="grid">
    <InputFilePicker
      {inputPath}
      disabled={isBusy}
      onSelect={handleInputSelect}
    />

    <AudioProbeInfo {probe} />

    <OutputSettings
      {outputFormat}
      {outputDir}
      {outputFilename}
      {mp3Bitrate}
      {splitByCue}
      disabled={isBusy}
      onFormatChange={handleFormatChange}
      onMp3BitrateChange={(bitrate) => {
        mp3Bitrate = bitrate;
      }}
      onOutputDirChange={(dir) => {
        outputDir = dir;
        lastError = null;
        successInfo = null;
        createdFiles = [];
      }}
      onOutputFilenameChange={(filename) => {
        outputFilename = filename;
        lastError = null;
        successInfo = null;
        createdFiles = [];
      }}
    />

    {#if outputFormat === "mp3"}
      <Mp3TagsSettings
        tags={mp3Tags}
        {coverPath}
        {splitByCue}
        disabled={isBusy}
        onTagsChange={(tags) => {
          mp3Tags = tags;
          lastError = null;
          successInfo = null;
          createdFiles = [];
        }}
        onCoverPathChange={(path) => {
          coverPath = path;
          lastError = null;
          successInfo = null;
          createdFiles = [];
        }}
      />
    {/if}

    {#if isFlacInput}
      <CueSplit
        {cuePath}
        {splitByCue}
        disabled={isBusy}
        onCueSelect={handleCueSelect}
        onSplitByCueChange={handleSplitByCueChange}
      />
    {/if}

    <StatusPanel
      {status}
      {errorMessage}
      {successMessage}
      {createdFiles}
      {ffmpegAvailable}
      {ffprobeAvailable}
    />
  </div>

  <div class="actions">
    <button
      type="button"
      class="primary"
      onclick={handleConvert}
      disabled={!canConvert}
    >
      {splitByCue ? $messages.actions.splitTracks : $messages.actions.convert}
    </button>
  </div>
</main>

<style>
  .app {
    max-width: 760px;
    margin: 0 auto;
    padding: 2rem 1.25rem 2.5rem;
  }

  header {
    margin-bottom: 1.5rem;
  }

  .header-row {
    display: flex;
    align-items: flex-start;
    gap: 1rem;
  }

  h1 {
    margin: 0 0 0.35rem;
    font-size: 1.75rem;
  }

  header p {
    margin: 0;
    color: var(--text-secondary);
  }

  .grid {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    margin-top: 1.25rem;
  }

  .primary {
    background: var(--accent);
    border-color: var(--accent);
    color: #ffffff;
  }

  .primary:hover:not(:disabled) {
    background: var(--accent-hover);
    border-color: var(--accent-hover);
  }
</style>
