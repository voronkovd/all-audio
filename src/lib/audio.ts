export function isFlacFile(path: string): boolean {
  return path.toLowerCase().endsWith(".flac");
}
