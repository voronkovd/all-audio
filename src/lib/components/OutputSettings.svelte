<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { messages } from "$lib/i18n/store";
  import { MP3_BITRATES, OUTPUT_FORMATS, type Mp3Bitrate, type OutputFormat } from "$lib/types";

  interface Props {
    outputFormat: OutputFormat;
    outputDir: string;
    outputFilename: string;
    mp3Bitrate: Mp3Bitrate;
    splitByCue: boolean;
    disabled?: boolean;
    onFormatChange: (format: OutputFormat) => void;
    onOutputDirChange: (dir: string) => void;
    onOutputFilenameChange: (filename: string) => void;
    onMp3BitrateChange: (bitrate: Mp3Bitrate) => void;
  }

  let {
    outputFormat,
    outputDir,
    outputFilename,
    mp3Bitrate,
    splitByCue,
    disabled = false,
    onFormatChange,
    onOutputDirChange,
    onOutputFilenameChange,
    onMp3BitrateChange,
  }: Props = $props();

  async function pickOutputDir() {
    const selected = await open({
      multiple: false,
      directory: true,
    });

    if (typeof selected === "string") {
      onOutputDirChange(selected);
    }
  }
</script>

<section class="card">
  <h2>{$messages.output.title}</h2>

  <label class="field">
    <span>{$messages.output.format}</span>
    <select
      value={outputFormat}
      onchange={(event) =>
        onFormatChange(
          (event.currentTarget as HTMLSelectElement).value as OutputFormat,
        )}
      {disabled}
    >
      {#each OUTPUT_FORMATS as format}
        <option value={format}>{format.toUpperCase()}</option>
      {/each}
    </select>
  </label>

  {#if outputFormat === "mp3"}
    <label class="field">
      <span>{$messages.output.mp3Bitrate}</span>
      <select
        value={mp3Bitrate}
        onchange={(event) =>
          onMp3BitrateChange(
            (event.currentTarget as HTMLSelectElement).value as Mp3Bitrate,
          )}
        {disabled}
      >
        {#each MP3_BITRATES as bitrate}
          <option value={bitrate}>{bitrate}</option>
        {/each}
      </select>
    </label>
  {/if}

  <div class="field">
    <span>{$messages.output.saveTo}</span>
    <div class="row">
      <input
        type="text"
        readonly
        value={outputDir}
        placeholder={$messages.output.noFolder}
      />
      <button type="button" onclick={pickOutputDir} {disabled}>{$messages.output.chooseFolder}</button>
    </div>
  </div>

  {#if !splitByCue}
    <label class="field">
      <span>{$messages.output.filename}</span>
      <input
        type="text"
        value={outputFilename}
        oninput={(event) =>
          onOutputFilenameChange(
            (event.currentTarget as HTMLInputElement).value,
          )}
        placeholder={$messages.output.filenamePlaceholder}
        {disabled}
      />
    </label>
  {/if}
</section>

<style>
  h2 {
    margin: 0 0 0.75rem;
    font-size: 1rem;
    font-weight: 600;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    margin-bottom: 0.9rem;
  }

  .field:last-child {
    margin-bottom: 0;
  }

  .field > span {
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  .row {
    display: flex;
    gap: 0.5rem;
  }

  .row input {
    flex: 1;
  }
</style>
