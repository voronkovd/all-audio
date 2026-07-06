<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { getMessages, messages } from "$lib/i18n/store";

  interface Props {
    inputPath: string;
    disabled?: boolean;
    onSelect: (path: string) => void;
  }

  let { inputPath, disabled = false, onSelect }: Props = $props();

  async function pickInputFile() {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: getMessages().input.audioFilter,
          extensions: [
            "mp3",
            "wav",
            "flac",
            "ogg",
            "opus",
            "m4a",
            "aac",
            "wma",
            "aiff",
            "aif",
            "alac",
            "webm",
            "mkv",
            "mp4",
          ],
        },
      ],
    });

    if (typeof selected === "string") {
      onSelect(selected);
    }
  }
</script>

<section class="card">
  <div class="card-header">
    <h2>{$messages.input.title}</h2>
    <button type="button" onclick={pickInputFile} {disabled}>{$messages.input.chooseFile}</button>
  </div>

  <p class="file-name" data-empty={!inputPath}>
    {inputPath || $messages.input.noFile}
  </p>
</section>

<style>
  .card {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
  }

  h2 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
  }

  .file-name {
    margin: 0;
    padding: 0.75rem 1rem;
    border-radius: 8px;
    background: var(--surface-muted);
    color: var(--text-secondary);
    word-break: break-all;
  }

  .file-name[data-empty="true"] {
    font-style: italic;
  }
</style>
