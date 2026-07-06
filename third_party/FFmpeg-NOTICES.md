# FFmpeg third-party notices

All Audio ships with **FFmpeg** and **FFprobe** binaries so users do not need
to install FFmpeg separately.

## Components

| Binary    | Purpose                                      |
| --------- | -------------------------------------------- |
| `ffmpeg`  | Audio format conversion and CUE track export |
| `ffprobe` | Reading input file metadata                  |

## Official project

- Website: [https://ffmpeg.org/](https://ffmpeg.org/)
- Legal / licensing overview: [https://ffmpeg.org/legal.html](https://ffmpeg.org/legal.html)

## Bundled build sources

| Platform | Source | License notes |
| -------- | ------ | ------------- |
| Windows x64 | [BtbN/FFmpeg-Builds](https://github.com/BtbN/FFmpeg-Builds) (`win64-lgpl`) | LGPL build |
| Linux x64 | [BtbN/FFmpeg-Builds](https://github.com/BtbN/FFmpeg-Builds) (`linux64-lgpl`) | LGPL build |
| macOS (Intel & Apple Silicon) | [eugeneware/ffmpeg-static](https://github.com/eugeneware/ffmpeg-static) (`b6.1.1`) | Static builds; may include GPL components — see upstream README/LICENSE files |

Binaries are downloaded at build time by `scripts/fetch-ffmpeg.sh` (`npm run fetch-ffmpeg`).
They are **not** downloaded at application runtime.

## How All Audio uses FFmpeg

All Audio executes FFmpeg/FFprobe as **separate processes**. It does not link
FFmpeg libraries into the application binary.

## Separation from All Audio

- **All Audio** source code is licensed under the **MIT License** (`/LICENSE`).
- **FFmpeg** is third-party software. License information is stored in this
  directory and is **not** merged into the project MIT license.

See also: [FFmpeg-LICENSE.txt](./FFmpeg-LICENSE.txt)

## Replacing bundled binaries

Developers can refresh binaries with:

```bash
npm run fetch-ffmpeg
```

If you substitute custom FFmpeg builds, ensure compliance with their licenses
and update this directory accordingly.

## macOS quarantine

On macOS, downloaded binaries in development may carry the quarantine attribute.
If execution fails, remove quarantine:

```bash
xattr -dr com.apple.quarantine src-tauri/resources/macos/
```

Release packages built in CI should not require this step for end users.
