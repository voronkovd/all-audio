<script lang="ts">
  import { LOCALE_OPTIONS, locale, messages, setLocale } from "$lib/i18n/store";
  import type { Locale } from "$lib/i18n/types";

  const languageLabels = $derived({
    en: $messages.language.en,
    ru: $messages.language.ru,
    de: $messages.language.de,
    fr: $messages.language.fr,
  });
</script>

<label class="language">
  <span>{$messages.language.label}</span>
  <select
    value={$locale}
    onchange={(event) =>
      setLocale((event.currentTarget as HTMLSelectElement).value as Locale)}
  >
    {#each LOCALE_OPTIONS as code}
      <option value={code}>{languageLabels[code]}</option>
    {/each}
  </select>
</label>

<style>
  .language {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-left: auto;
  }

  .language span {
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  .language select {
    width: auto;
    min-width: 8.5rem;
  }
</style>
