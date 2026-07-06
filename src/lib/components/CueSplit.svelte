<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { getMessages, messages } from "$lib/i18n/store";

  interface Props {
    cuePath: string;
    splitByCue: boolean;
    disabled?: boolean;
    onCueSelect: (path: string) => void;
    onSplitByCueChange: (enabled: boolean) => void;
  }

  let {
    cuePath,
    splitByCue,
    disabled = false,
    onCueSelect,
    onSplitByCueChange,
  }: Props = $props();

  async function pickCueFile() {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{ name: getMessages().cue.filter, extensions: ["cue"] }],
    });

    if (typeof selected === "string") {
      onCueSelect(selected);
    }
  }
</script>

<section class="card">
  <div class="card-header">
    <h2>{$messages.cue.title}</h2>
    <button type="button" onclick={pickCueFile} {disabled}>{$messages.cue.chooseFile}</button>
  </div>

  <p class="file-name" data-empty={!cuePath}>
    {cuePath || $messages.cue.noFile}
  </p>

  <label class="checkbox">
    <input
      type="checkbox"
      checked={splitByCue}
      disabled={disabled || !cuePath}
      onchange={(event) =>
        onSplitByCueChange((event.currentTarget as HTMLInputElement).checked)}
    />
    <span>{$messages.cue.splitCheckbox}</span>
  </label>

  <p class="warning">{$messages.cue.warning}</p>
</section>

<style>
  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    margin-bottom: 0.75rem;
  }

  h2 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
  }

  .file-name {
    margin: 0 0 0.75rem;
    padding: 0.75rem 1rem;
    border-radius: 8px;
    background: var(--surface-muted);
    color: var(--text-secondary);
    word-break: break-all;
  }

  .file-name[data-empty="true"] {
    font-style: italic;
  }

  .checkbox {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.75rem;
    font-size: 0.9rem;
  }

  .checkbox input {
    width: auto;
  }

  .warning {
    margin: 0;
    font-size: 0.875rem;
    color: #b45309;
  }
</style>
