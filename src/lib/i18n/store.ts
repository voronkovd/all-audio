import { derived, get, writable } from "svelte/store";

import { locales, LOCALE_OPTIONS } from "./locales";
import type { Locale, Messages } from "./types";

const STORAGE_KEY = "all-audio-locale";

export const locale = writable<Locale>("en");

export const messages = derived(locale, ($locale) => locales[$locale]);

export { LOCALE_OPTIONS };

export function getLocale(): Locale {
  return get(locale);
}

export function getMessages(): Messages {
  return get(messages);
}

export function setLocale(next: Locale) {
  locale.set(next);

  if (typeof localStorage !== "undefined") {
    localStorage.setItem(STORAGE_KEY, next);
  }

  if (typeof document !== "undefined") {
    document.documentElement.lang = next;
    document.title = locales[next].app.title;
  }
}

export function initLocale() {
  if (typeof window === "undefined") {
    return;
  }

  const saved = localStorage.getItem(STORAGE_KEY) as Locale | null;
  if (saved && saved in locales) {
    setLocale(saved);
    return;
  }

  const browser = navigator.language.slice(0, 2) as Locale;
  if (browser in locales) {
    setLocale(browser);
    return;
  }

  setLocale("en");
}

export function formatMessage(
  template: string,
  params: Record<string, string | number>,
): string {
  return Object.entries(params).reduce(
    (result, [key, value]) => result.replaceAll(`{${key}}`, String(value)),
    template,
  );
}
