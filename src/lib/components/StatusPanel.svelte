<script lang="ts">
  import { messages } from "$lib/i18n/store";
  import type { AppStatus } from "$lib/types";

  interface Props {
    status: AppStatus;
    errorMessage: string;
    successMessage: string;
    createdFiles: string[];
    ffmpegAvailable: boolean;
    ffprobeAvailable: boolean;
  }

  let {
    status,
    errorMessage,
    successMessage,
    createdFiles,
    ffmpegAvailable,
    ffprobeAvailable,
  }: Props = $props();

  const statusLabels: Record<AppStatus, string> = $derived({
    idle: $messages.status.idle,
    checking_ffmpeg: $messages.status.checkingFfmpeg,
    probing: $messages.status.probing,
    converting: $messages.status.converting,
    splitting: $messages.status.splitting,
    done: $messages.status.done,
    error: $messages.status.error,
  });
</script>

<section class="status card">
  <div class="status-row">
    <span class="label">{$messages.status.label}</span>
    <span class={`badge badge-${status}`}>{statusLabels[status]}</span>
  </div>

  {#if !ffmpegAvailable || !ffprobeAvailable}
    <p class="warning">
      {#if !ffmpegAvailable && !ffprobeAvailable}
        {$messages.status.ffmpegBothMissing}
      {:else if !ffmpegAvailable}
        {$messages.status.ffmpegMissing}
      {:else}
        {$messages.status.ffprobeMissing}
      {/if}
    </p>
  {/if}

  {#if errorMessage}
    <p class="error">{errorMessage}</p>
  {/if}

  {#if successMessage}
    <p class="success">{successMessage}</p>
  {/if}

  {#if createdFiles.length > 0}
    <ul class="file-list">
      {#each createdFiles as file}
        <li>{file}</li>
      {/each}
    </ul>
  {/if}
</section>

<style>
  .status-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
  }

  .label {
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  .badge {
    display: inline-flex;
    align-items: center;
    padding: 0.25rem 0.75rem;
    border-radius: 999px;
    font-size: 0.875rem;
    font-weight: 600;
  }

  .badge-idle {
    background: #e8edf5;
    color: #334155;
  }

  .badge-checking_ffmpeg,
  .badge-probing,
  .badge-converting,
  .badge-splitting {
    background: #dbeafe;
    color: #1d4ed8;
  }

  .badge-done {
    background: #dcfce7;
    color: #166534;
  }

  .badge-error {
    background: #fee2e2;
    color: #b91c1c;
  }

  .warning,
  .error,
  .success {
    margin: 0.75rem 0 0;
    font-size: 0.9rem;
  }

  .warning {
    color: #b45309;
  }

  .error {
    color: #b91c1c;
  }

  .success {
    color: #166534;
    word-break: break-all;
  }

  .file-list {
    margin: 0.75rem 0 0;
    padding-left: 1.25rem;
    color: #166534;
    font-size: 0.875rem;
    word-break: break-all;
  }

  .file-list li + li {
    margin-top: 0.35rem;
  }
</style>
