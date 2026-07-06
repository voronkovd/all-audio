use std::path::{Path, PathBuf};
use std::process::Command;

use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

use crate::error::{CommandError, CommandResult};
use crate::models::FfmpegAvailability;

/// Resolves paths to bundled FFmpeg / FFprobe binaries via Tauri resources.
///
/// Today always uses embedded binaries. The module is structured so a future
/// system-FFmpeg fallback can be added without touching call sites.
#[derive(Clone, Debug)]
pub struct FfmpegProvider {
    ffmpeg_path: PathBuf,
    ffprobe_path: PathBuf,
}

impl FfmpegProvider {
    pub fn initialize(handle: &AppHandle) -> CommandResult<Self> {
        let ffmpeg_path = resolve_resource(handle, ffmpeg_resource_name())?;
        let ffprobe_path = resolve_resource(handle, ffprobe_resource_name())?;

        validate_binary("ffmpeg", &ffmpeg_path)?;
        validate_binary("ffprobe", &ffprobe_path)?;

        Ok(Self {
            ffmpeg_path,
            ffprobe_path,
        })
    }

    pub fn get_ffmpeg_path(&self) -> &Path {
        &self.ffmpeg_path
    }

    pub fn get_ffprobe_path(&self) -> &Path {
        &self.ffprobe_path
    }

    pub fn check_availability(&self) -> FfmpegAvailability {
        let ffmpeg = run_version_check(&self.ffmpeg_path);
        let ffprobe = run_version_check(&self.ffprobe_path);
        FfmpegAvailability { ffmpeg, ffprobe }
    }
}

fn resolve_resource(handle: &AppHandle, resource_name: &str) -> CommandResult<PathBuf> {
    handle
        .path()
        .resolve(resource_name, BaseDirectory::Resource)
        .map_err(|error| {
            CommandError::new(
                "bundled_ffmpeg_resolve_failed",
                format!("Failed to resolve bundled FFmpeg resource \"{resource_name}\": {error}"),
            )
        })
}

fn validate_binary(name: &str, path: &Path) -> CommandResult<()> {
    if !path.exists() {
        return Err(CommandError::new(
            if name == "ffmpeg" {
                "bundled_ffmpeg_missing"
            } else {
                "bundled_ffprobe_missing"
            },
            format!("Bundled {name} was not found at {}", path.display()),
        ));
    }

    if !path.is_file() {
        return Err(CommandError::new(
            "bundled_ffmpeg_invalid",
            format!("Bundled {name} path is not a file: {}", path.display()),
        ));
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let metadata = path.metadata().map_err(|error| {
            CommandError::new(
                "bundled_ffmpeg_invalid",
                format!("Cannot read bundled {name} metadata: {error}"),
            )
        })?;

        if metadata.permissions().mode() & 0o111 == 0 {
            let mut permissions = metadata.permissions();
            permissions.set_mode(metadata.permissions().mode() | 0o755);
            std::fs::set_permissions(path, permissions).map_err(|error| {
                CommandError::new(
                    "bundled_ffmpeg_not_executable",
                    format!("Bundled {name} is not executable and could not be fixed: {error}"),
                )
            })?;
        }
    }

    Ok(())
}

fn run_version_check(path: &Path) -> bool {
    Command::new(path)
        .arg("-version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(target_os = "windows")]
fn ffmpeg_resource_name() -> &'static str {
    "windows/ffmpeg.exe"
}

#[cfg(target_os = "windows")]
fn ffprobe_resource_name() -> &'static str {
    "windows/ffprobe.exe"
}

#[cfg(target_os = "linux")]
fn ffmpeg_resource_name() -> &'static str {
    "linux/ffmpeg"
}

#[cfg(target_os = "linux")]
fn ffprobe_resource_name() -> &'static str {
    "linux/ffprobe"
}

#[cfg(target_os = "macos")]
fn ffmpeg_resource_name() -> &'static str {
    "macos/ffmpeg"
}

#[cfg(target_os = "macos")]
fn ffprobe_resource_name() -> &'static str {
    "macos/ffprobe"
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn ffmpeg_resource_name() -> &'static str {
    "linux/ffmpeg"
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn ffprobe_resource_name() -> &'static str {
    "linux/ffprobe"
}
