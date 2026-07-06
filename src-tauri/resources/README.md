Bundled FFmpeg binaries are not stored in git.

Run from the repository root:

```bash
npm run fetch-ffmpeg
```

This downloads platform-specific binaries into:

- `windows/ffmpeg.exe`, `windows/ffprobe.exe` — [BtbN/FFmpeg-Builds](https://github.com/BtbN/FFmpeg-Builds) (LGPL)
- `linux/ffmpeg`, `linux/ffprobe` — BtbN/FFmpeg-Builds (LGPL)
- `macos/ffmpeg`, `macos/ffprobe` — [eugeneware/ffmpeg-static](https://github.com/eugeneware/ffmpeg-static) `b6.1.1`

The release workflow runs the same script before building installers.
