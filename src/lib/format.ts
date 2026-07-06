export function formatDuration(seconds: number | null): string {
  if (seconds === null || Number.isNaN(seconds)) {
    return "—";
  }

  const totalSeconds = Math.max(0, Math.round(seconds));
  const minutes = Math.floor(totalSeconds / 60);
  const remainingSeconds = totalSeconds % 60;

  return `${minutes}:${remainingSeconds.toString().padStart(2, "0")}`;
}

export function formatBitrate(bitrate: number | null): string {
  if (bitrate === null) {
    return "—";
  }

  return `${Math.round(bitrate / 1000)} kbps`;
}

export function basename(path: string): string {
  const parts = path.split(/[/\\]/);
  return parts[parts.length - 1] ?? path;
}

export function defaultOutputName(inputPath: string, format: string): string {
  const name = basename(inputPath);
  const dotIndex = name.lastIndexOf(".");

  if (dotIndex <= 0) {
    return `converted.${format}`;
  }

  return `${name.slice(0, dotIndex)}.${format}`;
}
