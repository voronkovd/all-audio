#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RESOURCES_DIR="$ROOT_DIR/src-tauri/resources"
FFMPEG_STATIC_TAG="b6.1.1"
FFMPEG_STATIC_BASE="https://github.com/eugeneware/ffmpeg-static/releases/download/${FFMPEG_STATIC_TAG}"
BTBN_BASE="https://github.com/BtbN/FFmpeg-Builds/releases/download/latest"

detect_platform() {
  case "$(uname -s)" in
    Linux)
      PLATFORM="linux"
      ;;
    Darwin)
      PLATFORM="macos"
      if [ "$(uname -m)" = "arm64" ]; then
        MAC_ARCH="arm64"
      else
        MAC_ARCH="x64"
      fi
      ;;
    MINGW* | MSYS* | CYGWIN* | Windows_NT)
      PLATFORM="windows"
      ;;
    *)
      echo "Unsupported platform: $(uname -s)" >&2
      exit 1
      ;;
  esac
}

already_present() {
  case "$PLATFORM" in
    windows)
      [ -f "$RESOURCES_DIR/windows/ffmpeg.exe" ] && [ -f "$RESOURCES_DIR/windows/ffprobe.exe" ]
      ;;
    linux | macos)
      [ -f "$RESOURCES_DIR/$PLATFORM/ffmpeg" ] && [ -f "$RESOURCES_DIR/$PLATFORM/ffprobe" ]
      ;;
    *)
      false
      ;;
  esac
}

download_btbn_archive() {
  local archive_name="$1"
  local archive_path
  archive_path="$(mktemp)"
  curl -fsSL "${BTBN_BASE}/${archive_name}" -o "$archive_path"
  printf '%s' "$archive_path"
}

extract_btbn_archive() {
  local archive_path="$1"
  local extract_dir
  extract_dir="$(mktemp -d)"

  case "$archive_path" in
    *.zip)
      unzip -q "$archive_path" -d "$extract_dir"
      ;;
    *.tar.xz)
      tar -xf "$archive_path" -C "$extract_dir"
      ;;
    *)
      echo "Unsupported archive format: $archive_path" >&2
      exit 1
      ;;
  esac

  local bin_dir
  bin_dir="$(find "$extract_dir" -type d -name bin | head -n 1)"
  if [ -z "$bin_dir" ]; then
    echo "Could not find bin/ directory in BtbN FFmpeg archive" >&2
    exit 1
  fi

  printf '%s' "$bin_dir"
}

install_windows_binaries() {
  local archive_path
  archive_path="$(download_btbn_archive "ffmpeg-master-latest-win64-lgpl.zip")"
  local bin_dir
  bin_dir="$(extract_btbn_archive "$archive_path")"

  mkdir -p "$RESOURCES_DIR/windows"
  install -m 755 "$bin_dir/ffmpeg.exe" "$RESOURCES_DIR/windows/ffmpeg.exe"
  install -m 755 "$bin_dir/ffprobe.exe" "$RESOURCES_DIR/windows/ffprobe.exe"

  rm -f "$archive_path"
}

install_linux_binaries() {
  local archive_path
  archive_path="$(download_btbn_archive "ffmpeg-master-latest-linux64-lgpl.tar.xz")"
  local bin_dir
  bin_dir="$(extract_btbn_archive "$archive_path")"

  mkdir -p "$RESOURCES_DIR/linux"
  install -m 755 "$bin_dir/ffmpeg" "$RESOURCES_DIR/linux/ffmpeg"
  install -m 755 "$bin_dir/ffprobe" "$RESOURCES_DIR/linux/ffprobe"

  rm -f "$archive_path"
}

install_macos_binaries() {
  local ffmpeg_asset="ffmpeg-darwin-${MAC_ARCH}"
  local ffprobe_asset="ffprobe-darwin-${MAC_ARCH}"
  local tmp_dir
  tmp_dir="$(mktemp -d)"

  curl -fsSL "${FFMPEG_STATIC_BASE}/${ffmpeg_asset}" -o "$tmp_dir/ffmpeg"
  curl -fsSL "${FFMPEG_STATIC_BASE}/${ffprobe_asset}" -o "$tmp_dir/ffprobe"

  mkdir -p "$RESOURCES_DIR/macos"
  install -m 755 "$tmp_dir/ffmpeg" "$RESOURCES_DIR/macos/ffmpeg"
  install -m 755 "$tmp_dir/ffprobe" "$RESOURCES_DIR/macos/ffprobe"

  rm -rf "$tmp_dir"
}

main() {
  detect_platform

  if already_present; then
    echo "Bundled FFmpeg already present for $PLATFORM — skipping download."
    exit 0
  fi

  echo "Downloading bundled FFmpeg for $PLATFORM..."

  case "$PLATFORM" in
    windows)
      install_windows_binaries
      ;;
    linux)
      install_linux_binaries
      ;;
    macos)
      install_macos_binaries
      ;;
  esac

  echo "Done. Bundled FFmpeg installed into $RESOURCES_DIR/$PLATFORM"
}

main "$@"
