<script lang="ts">
  import { messages } from "$lib/i18n/store";
  import type { AudioProbe } from "$lib/types";
  import { formatBitrate, formatDuration } from "$lib/format";

  interface Props {
    probe: AudioProbe | null;
  }

  let { probe }: Props = $props();
</script>

{#if probe}
  <section class="probe card">
    <h2>{$messages.probe.title}</h2>
    <dl>
      <div>
        <dt>{$messages.probe.duration}</dt>
        <dd>{formatDuration(probe.duration)}</dd>
      </div>
      <div>
        <dt>{$messages.probe.codec}</dt>
        <dd>{probe.codec ?? $messages.probe.empty}</dd>
      </div>
      <div>
        <dt>{$messages.probe.sampleRate}</dt>
        <dd>{probe.sample_rate ? `${probe.sample_rate} Hz` : $messages.probe.empty}</dd>
      </div>
      <div>
        <dt>{$messages.probe.channels}</dt>
        <dd>{probe.channels ?? $messages.probe.empty}</dd>
      </div>
      <div>
        <dt>{$messages.probe.bitrate}</dt>
        <dd>{formatBitrate(probe.bitrate)}</dd>
      </div>
    </dl>
  </section>
{/if}

<style>
  .probe h2 {
    margin: 0 0 0.75rem;
    font-size: 1rem;
    font-weight: 600;
  }

  dl {
    margin: 0;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
    gap: 0.75rem;
  }

  dt {
    margin: 0;
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--text-secondary);
  }

  dd {
    margin: 0.25rem 0 0;
    font-weight: 600;
  }
</style>
