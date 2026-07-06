# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-07-06

### Added

- Cross-platform desktop app (Linux, Windows, macOS) built with Tauri 2, Rust, Svelte, and TypeScript
- **Built-in FFmpeg and FFprobe** (LGPL builds from BtbN/FFmpeg-Builds) — no manual FFmpeg install for end users
- `ffmpeg_provider` module for resolving bundled binary paths (extensible for future system-FFmpeg fallback)
- `npm run fetch-ffmpeg` script and CI step to embed platform binaries in release packages
- Third-party license notices in `third_party/`
- Audio conversion between MP3, WAV, FLAC, OGG, Opus, M4A, and AAC via FFmpeg CLI
- FFprobe-based source metadata display (duration, codec, sample rate, channels, bitrate)
- MP3 bitrate selection: 128k, 192k, 256k, 320k (default 320k)
- FLAC + CUE track splitting with multi-`FILE` CUE sheet support
- UI localization: English, Russian, German, French
- Non-blocking conversion using `spawn_blocking`
- Path and extension validation; refusal to overwrite existing output files
- GitHub Actions release workflow for Linux, Windows, and macOS

### Security

- Output filename validation blocks path separators and `..`
- Input/output same-file check before conversion
- Rollback of partially created files on CUE split failure

[0.1.0]: https://github.com/voronkovd/all-audio/releases/tag/v0.1.0
