<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { messages } from "$lib/i18n/store";
  import type { Mp3Tags } from "$lib/mp3";

  interface Props {
    tags: Mp3Tags;
    coverPath: string;
    splitByCue: boolean;
    disabled?: boolean;
    onTagsChange: (tags: Mp3Tags) => void;
    onCoverPathChange: (path: string) => void;
  }

  let {
    tags,
    coverPath,
    splitByCue,
    disabled = false,
    onTagsChange,
    onCoverPathChange,
  }: Props = $props();

  function updateField<K extends keyof Mp3Tags>(field: K, value: string) {
    onTagsChange({ ...tags, [field]: value });
  }

  async function pickCover() {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: $messages.mp3Tags.coverFilter,
          extensions: ["jpg", "jpeg", "png", "webp", "bmp"],
        },
      ],
    });

    if (typeof selected === "string") {
      onCoverPathChange(selected);
    }
  }
</script>

<section class="card">
  <h2>{$messages.mp3Tags.title}</h2>
  <p class="hint">{$messages.mp3Tags.hint}</p>

  {#if splitByCue}
    <p class="note">{$messages.mp3Tags.cueHint}</p>
  {/if}

  <div class="grid">
    <label class="field">
      <span>{$messages.mp3Tags.titleField}</span>
      <input
        type="text"
        value={tags.title}
        oninput={(event) =>
          updateField("title", (event.currentTarget as HTMLInputElement).value)}
        placeholder={$messages.mp3Tags.titlePlaceholder}
        {disabled}
      />
    </label>

    <label class="field">
      <span>{$messages.mp3Tags.artist}</span>
      <input
        type="text"
        value={tags.artist}
        oninput={(event) =>
          updateField("artist", (event.currentTarget as HTMLInputElement).value)}
        placeholder={$messages.mp3Tags.artistPlaceholder}
        {disabled}
      />
    </label>

    <label class="field">
      <span>{$messages.mp3Tags.album}</span>
      <input
        type="text"
        value={tags.album}
        oninput={(event) =>
          updateField("album", (event.currentTarget as HTMLInputElement).value)}
        placeholder={$messages.mp3Tags.albumPlaceholder}
        {disabled}
      />
    </label>

    <label class="field">
      <span>{$messages.mp3Tags.albumArtist}</span>
      <input
        type="text"
        value={tags.albumArtist}
        oninput={(event) =>
          updateField(
            "albumArtist",
            (event.currentTarget as HTMLInputElement).value,
          )}
        placeholder={$messages.mp3Tags.albumArtistPlaceholder}
        {disabled}
      />
    </label>

    <label class="field">
      <span>{$messages.mp3Tags.year}</span>
      <input
        type="text"
        value={tags.year}
        oninput={(event) =>
          updateField("year", (event.currentTarget as HTMLInputElement).value)}
        placeholder={$messages.mp3Tags.yearPlaceholder}
        {disabled}
      />
    </label>

    <label class="field">
      <span>{$messages.mp3Tags.genre}</span>
      <input
        type="text"
        value={tags.genre}
        oninput={(event) =>
          updateField("genre", (event.currentTarget as HTMLInputElement).value)}
        placeholder={$messages.mp3Tags.genrePlaceholder}
        {disabled}
      />
    </label>

    {#if !splitByCue}
      <label class="field">
        <span>{$messages.mp3Tags.track}</span>
        <input
          type="text"
          value={tags.track}
          oninput={(event) =>
            updateField("track", (event.currentTarget as HTMLInputElement).value)}
          placeholder={$messages.mp3Tags.trackPlaceholder}
          {disabled}
        />
      </label>
    {/if}
  </div>

  <div class="field cover-field">
    <span>{$messages.mp3Tags.cover}</span>
    <div class="row">
      <input
        type="text"
        readonly
        value={coverPath}
        placeholder={$messages.mp3Tags.noCover}
      />
      <button type="button" onclick={pickCover} {disabled}>
        {$messages.mp3Tags.chooseCover}
      </button>
      {#if coverPath}
        <button
          type="button"
          class="secondary"
          onclick={() => onCoverPathChange("")}
          {disabled}
        >
          {$messages.mp3Tags.clearCover}
        </button>
      {/if}
    </div>
  </div>
</section>

<style>
  h2 {
    margin: 0 0 0.35rem;
    font-size: 1rem;
    font-weight: 600;
  }

  .hint,
  .note {
    margin: 0 0 0.75rem;
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  .note {
    color: #b45309;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 0.75rem;
  }

  @media (max-width: 640px) {
    .grid {
      grid-template-columns: 1fr;
    }
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  .field > span {
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  .cover-field {
    margin-top: 0.9rem;
  }

  .row {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .row input {
    flex: 1;
    min-width: 12rem;
  }

  .secondary {
    background: transparent;
  }
</style>
