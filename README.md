# All Audio

<!-- TODO: Add project logo -->

> Cross-platform open-source desktop app for converting audio files between popular formats.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Release](https://github.com/voronkovd/all-audio/actions/workflows/release.yml/badge.svg)](https://github.com/voronkovd/all-audio/actions/workflows/release.yml)

All Audio is a lightweight desktop converter built with **Tauri 2**, **Rust**, **Svelte**, and **TypeScript**. Conversion and analysis use **bundled FFmpeg** and **FFprobe** — no separate installation required for end users.

<!-- TODO: Add screenshots -->

## Features

- Pick an input audio file and view metadata from FFprobe
- Convert to **MP3**, **WAV**, **FLAC**, **OGG**, **Opus**, **M4A**, or **AAC**
- **MP3 bitrate** selection: 128k, 192k, 256k, 320k (default: 320k)
- **FLAC + CUE split** — split album FLAC files into separate tracks
- Non-blocking conversion (UI stays responsive)
- Path and extension validation; refuses to overwrite existing files
- **Built-in FFmpeg and FFprobe** — works out of the box on Windows, Linux, and macOS
- UI localization: **English**, **Russian**, **German**, **French**

## Built-in FFmpeg

All Audio ships with **FFmpeg** and **FFprobe** binaries embedded in the application package.

- End users do **not** need to install FFmpeg manually
- The app never relies on system `PATH` for conversion
- LGPL builds are downloaded at build time via `npm run fetch-ffmpeg` (also run automatically in CI)
- Windows and Linux use **LGPL** builds from [BtbN/FFmpeg-Builds](https://github.com/BtbN/FFmpeg-Builds)
- macOS uses static builds from [eugeneware/ffmpeg-static](https://github.com/eugeneware/ffmpeg-static) — see [`third_party/`](third_party/) for license details

Supported bundled platforms:

| Platform | Architecture |
|----------|--------------|
| Windows | x64 |
| Linux | x64 |
| macOS | Apple Silicon (arm64) |
| macOS | Intel (x64, local builds) |

## Supported operating systems

| OS | Status |
|----|--------|
| Linux | Supported (CI: Ubuntu 22.04) |
| Windows | Supported |
| macOS | Supported (Intel & Apple Silicon via CI) |

## Supported formats

### Input

Common audio extensions including: `mp3`, `wav`, `flac`, `ogg`, `opus`, `m4a`, `aac`, `wma`, `aiff`, `alac`, `ape`, `wv`, `webm`, `mkv`, `mp4`, and more.

### Output

`mp3`, `wav`, `flac`, `ogg`, `opus`, `m4a`, `aac`

## Requirements

### End users (releases)

No additional dependencies. Download a release installer for your platform.

### Developers (build from source)

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://www.rust-lang.org/tools/install) (stable)
- `curl`, `unzip`, and `tar` (used by `npm run fetch-ffmpeg`)

Bundled FFmpeg binaries are fetched automatically before production builds. For local development, run once:

```bash
npm run fetch-ffmpeg
```

## Installation

### Pre-built releases

Latest release: **[v0.1.1](https://github.com/voronkovd/all-audio/releases/tag/v0.1.1)**  
All downloads: [GitHub Releases](https://github.com/voronkovd/all-audio/releases)

> **Note:** Installer filenames use app version `0.1.0` from `tauri.conf.json`; the git release tag is `v0.1.1`.

| Platform | File | Download |
|----------|------|----------|
| **macOS** (Apple Silicon) | `.dmg` | [All.Audio_0.1.0_aarch64.dmg](https://github.com/voronkovd/all-audio/releases/download/v0.1.1/All.Audio_0.1.0_aarch64.dmg) |
| **macOS** (Apple Silicon) | `.app.tar.gz` | [All.Audio_aarch64.app.tar.gz](https://github.com/voronkovd/all-audio/releases/download/v0.1.1/All.Audio_aarch64.app.tar.gz) |
| **Windows** (x64) | `.msi` | [All.Audio_0.1.0_x64_en-US.msi](https://github.com/voronkovd/all-audio/releases/download/v0.1.1/All.Audio_0.1.0_x64_en-US.msi) |
| **Windows** (x64) | setup `.exe` | [All.Audio_0.1.0_x64-setup.exe](https://github.com/voronkovd/all-audio/releases/download/v0.1.1/All.Audio_0.1.0_x64-setup.exe) |
| **Linux** (x64) | `.deb` | [All.Audio_0.1.0_amd64.deb](https://github.com/voronkovd/all-audio/releases/download/v0.1.1/All.Audio_0.1.0_amd64.deb) |
| **Linux** (x64) | `.AppImage` | [All.Audio_0.1.0_amd64.AppImage](https://github.com/voronkovd/all-audio/releases/download/v0.1.1/All.Audio_0.1.0_amd64.AppImage) |
| **Linux** (x64) | `.rpm` | [All.Audio-0.1.0-1.x86_64.rpm](https://github.com/voronkovd/all-audio/releases/download/v0.1.1/All.Audio-0.1.0-1.x86_64.rpm) |

Release builds are **unsigned**. See [First launch](#first-launch) below if the OS blocks the app.

### First launch

#### macOS

After downloading, macOS may show **«Приложение повреждено»** or refuse to open the app. The app is not broken — Gatekeeper blocks unsigned builds downloaded from the browser.

1. Open **Terminal** (Программы → Утилиты → Терминал).
2. Remove the quarantine flag (adjust the path if needed):

```bash
xattr -cr ~/Downloads/All\ Audio.app
```

If the app is already in Applications:

```bash
xattr -cr /Applications/All\ Audio.app
```

3. Right-click `All Audio.app` → **Open** → **Open** (do not use double-click the first time).

If it still does not open, ad-hoc sign the app and try again:

```bash
codesign --force --deep --sign - ~/Downloads/All\ Audio.app
xattr -cr ~/Downloads/All\ Audio.app
open ~/Downloads/All\ Audio.app
```

For `.dmg`: open the disk image, drag the app to Applications, then run `xattr` on `/Applications/All Audio.app`.

#### Windows

SmartScreen may warn about an unknown publisher. Click **More info** → **Run anyway**.

For `.msi`: right-click → **Properties** → check **Unblock** at the bottom → OK → install again.

#### Linux

For `.AppImage`:

```bash
chmod +x All.Audio_0.1.0_amd64.AppImage
./All.Audio_0.1.0_amd64.AppImage
```

### Build from source

```bash
git clone https://github.com/voronkovd/all-audio.git
cd all-audio
npm install
npm run fetch-ffmpeg   # download bundled FFmpeg (once per platform)
npm run tauri dev      # development
npm run tauri build    # production bundle
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and PR requirements.

## MP3 Bitrate

When the output format is **MP3**, choose a bitrate from the dropdown:

| Bitrate | Use case |
|---------|----------|
| 128k | Smaller files, speech/podcasts |
| 192k | Balanced quality/size |
| 256k | High quality |
| 320k | Maximum quality (default) |

The value is passed to FFmpeg as `-b:a <bitrate>` for single-file conversion and for each track during CUE splitting.

## CUE Split

If the input is a `.flac` album, the **CUE Split** panel appears:

1. Choose a `.cue` file
2. Enable **Split FLAC by CUE**
3. Select the **matching** FLAC referenced in the CUE sheet
4. Choose output folder and format
5. Click **Split tracks**

Output filenames follow the pattern:

```text
01 - Track Title.mp3
02 - Another Song.flac
```

### Multi-file CUE sheets

Many CUE sheets reference multiple files (e.g. `Side_A.flac` and `Side_B.flac`). The app uses only tracks from the `FILE` block that matches the selected input FLAC.

### Example

```cue
FILE "Side_A.flac" WAVE
  TRACK 01 AUDIO
    TITLE "Intro"
    INDEX 01 00:00:00
FILE "Side_B.flac" WAVE
  TRACK 02 AUDIO
    TITLE "Outro"
    INDEX 01 00:00:00
```

Select `Side_A.flac` to split track 01, or `Side_B.flac` for track 02.

### CUE limitations (v0.1.0)

- Basic CUE parsing; only `INDEX 01` is supported
- Pregap and `INDEX 00` are not supported yet
- Splitting is experimental — verify track boundaries after export

## Localization

The UI supports English, Russian, German, and French. Language is saved in `localStorage` and auto-detected from the browser locale on first launch.

## Project structure

```text
src/                  Svelte + TypeScript frontend
  lib/
    api.ts            Tauri command wrappers
    i18n/             UI translations (en, ru, de, fr)
    components/       UI components
src-tauri/            Rust backend
  src/
    commands.rs       Tauri command handlers
    cue.rs            CUE parser
    ffmpeg.rs         FFmpeg / FFprobe integration (via ffmpeg_provider)
    ffmpeg_provider.rs  Bundled FFmpeg path resolution
    validation.rs     Path and format validation
```

## Tauri commands

| Command | Description |
|---------|-------------|
| `check_ffmpeg_available` | Checks whether bundled `ffmpeg` and `ffprobe` are available |
| `probe_audio_file` | Reads duration, codec, sample rate, channels, bitrate |
| `convert_audio` | Converts one file or splits FLAC by CUE |

## Release builds

Tagged releases are built automatically with GitHub Actions for Linux, Windows, and macOS.

```bash
git tag v0.1.0
git push origin v0.1.0
```

See [CHANGELOG.md](CHANGELOG.md) for version history. Release builds are currently **unsigned**.

## Roadmap

- [ ] Batch conversion for multiple files
- [ ] Progress reporting during conversion
- [ ] More CUE features (pregap, INDEX 00)
- [ ] Drag-and-drop input files
- [ ] Overwrite confirmation dialog
- [ ] Conversion history and presets
- [ ] App settings (default output folder, format)
- [ ] Optional system FFmpeg fallback
- [ ] Project logo and screenshots

## Third-party software

This project uses **[FFmpeg](https://ffmpeg.org/)** for audio conversion and metadata probing.

- Official website: [https://ffmpeg.org/](https://ffmpeg.org/)
- Bundled binaries are **LGPL** on Windows/Linux (BtbN) and static builds on macOS — see [`third_party/`](third_party/)
- License and notices: [`third_party/`](third_party/)

The All Audio application itself remains under the [MIT License](LICENSE). FFmpeg licensing is documented separately in `third_party/` and is not merged into the project license.

## FAQ

### Why does macOS say the app is damaged?

Release builds are not signed with an Apple Developer certificate. After downloading from Chrome or another browser, macOS sets a quarantine flag. Run `xattr -cr` on the `.app` bundle — see [First launch](#first-launch).

### Why does the app say FFmpeg is not found?

Release builds include bundled FFmpeg. If you see this error in a **development** build, run `npm run fetch-ffmpeg` once, then restart the app. If the error appears in an official release, reinstall the application or report a bug.

### Can I convert without an internet connection?

Yes. All processing is local. FFmpeg runs on your machine.

### Why won't the app overwrite my output file?

By design — v0.1.0 refuses to overwrite existing files to prevent accidental data loss. Choose a different filename or delete the existing file first.

### CUE split created wrong track boundaries

CUE splitting depends on accurate `INDEX 01` values in the CUE sheet. Verify the CUE file and try re-encoding from the source FLAC. Pregap is not supported yet.

### Does All Audio work with Unicode file paths?

Yes. Paths are passed to FFmpeg as native OS strings, supporting Unicode filenames on all platforms.

### Which license applies?

All Audio source code is [MIT](LICENSE). Bundled FFmpeg is third-party software under **LGPL** — see [`third_party/`](third_party/).

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) and our [Code of Conduct](CODE_OF_CONDUCT.md) before opening a pull request.

Found a security issue? See [SECURITY.md](SECURITY.md).

## License

MIT © All Audio contributors — see [LICENSE](LICENSE).
