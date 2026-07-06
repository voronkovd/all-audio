import { de } from "./de";
import { en } from "./en";
import { fr } from "./fr";
import { ru } from "./ru";
import type { Locale, Messages } from "../types";

export const locales: Record<Locale, Messages> = {
  en,
  ru,
  de,
  fr,
};

export const LOCALE_OPTIONS: Locale[] = ["en", "ru", "de", "fr"];
