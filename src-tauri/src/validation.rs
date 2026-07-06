use std::path::{Path, PathBuf};

use crate::error::{CommandError, CommandResult};

pub const OUTPUT_FORMATS: &[&str] = &["mp3", "wav", "flac", "ogg", "opus", "m4a", "aac"];
pub const MP3_BITRATES: &[&str] = &["128k", "192k", "256k", "320k"];

const INPUT_AUDIO_EXTENSIONS: &[&str] = &[
    "mp3", "wav", "flac", "ogg", "opus", "m4a", "aac", "wma", "aiff", "aif", "alac", "ape", "wv",
    "webm", "mkv", "mp4", "m4b", "caf", "ac3", "dts",
];

pub fn validate_input_file(path: &str) -> CommandResult<PathBuf> {
    let path = PathBuf::from(path);

    if path.as_os_str().is_empty() {
        return Err(CommandError::new(
            "invalid_input_path",
            "Input file path is empty",
        ));
    }

    if !path.exists() {
        return Err(CommandError::new(
            "input_not_found",
            "Input file does not exist",
        ));
    }

    if !path.is_file() {
        return Err(CommandError::new(
            "invalid_input_path",
            "Input path is not a file",
        ));
    }

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();

    if extension.is_empty() || !INPUT_AUDIO_EXTENSIONS.contains(&extension.as_str()) {
        return Err(CommandError::new(
            "unsupported_input_extension",
            "Unsupported input file extension",
        ));
    }

    Ok(path)
}

pub fn validate_output_format(format: &str) -> CommandResult<String> {
    let format = format.trim().to_ascii_lowercase();

    if !OUTPUT_FORMATS.contains(&format.as_str()) {
        return Err(CommandError::new(
            "unsupported_output_format",
            "Unsupported output format",
        ));
    }

    Ok(format)
}

pub fn validate_output_dir(path: &str) -> CommandResult<PathBuf> {
    let path = PathBuf::from(path);

    if path.as_os_str().is_empty() {
        return Err(CommandError::new(
            "invalid_output_dir",
            "Output directory path is empty",
        ));
    }

    if !path.exists() {
        return Err(CommandError::new(
            "output_dir_not_found",
            "Output directory does not exist",
        ));
    }

    if !path.is_dir() {
        return Err(CommandError::new(
            "invalid_output_dir",
            "Output path is not a directory",
        ));
    }

    Ok(path)
}

pub fn validate_output_filename(filename: &str) -> CommandResult<String> {
    let filename = filename.trim();

    if filename.is_empty() {
        return Err(CommandError::new(
            "invalid_output_filename",
            "Output filename is empty",
        ));
    }

    if filename.contains('/') || filename.contains('\\') {
        return Err(CommandError::new(
            "invalid_output_filename",
            "Output filename must not contain path separators",
        ));
    }

    if filename.contains("..") {
        return Err(CommandError::new(
            "invalid_output_filename",
            "Output filename is invalid",
        ));
    }

    Ok(filename.to_string())
}

pub fn build_output_path(
    output_dir: &Path,
    output_filename: &str,
    output_format: &str,
) -> CommandResult<PathBuf> {
    let mut filename = output_filename.to_string();
    let expected_extension = format!(".{output_format}");

    if let Some(extension) = Path::new(&filename)
        .extension()
        .and_then(|ext| ext.to_str())
    {
        if extension.to_ascii_lowercase() != output_format {
            return Err(CommandError::new(
                "output_extension_mismatch",
                "Output filename extension does not match selected format",
            ));
        }
    } else {
        filename.push_str(&expected_extension);
    }

    let output_path = output_dir.join(filename);

    if output_path.exists() {
        return Err(CommandError::new(
            "output_file_exists",
            "Output file already exists",
        ));
    }

    Ok(output_path)
}

pub fn is_flac_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("flac"))
        .unwrap_or(false)
}

pub fn validate_cue_file(path: &str) -> CommandResult<PathBuf> {
    let path = PathBuf::from(path);

    if path.as_os_str().is_empty() {
        return Err(CommandError::new(
            "invalid_cue_path",
            "CUE file path is empty",
        ));
    }

    if !path.exists() {
        return Err(CommandError::new(
            "cue_not_found",
            "CUE file does not exist",
        ));
    }

    if !path.is_file() {
        return Err(CommandError::new(
            "invalid_cue_path",
            "CUE path is not a file",
        ));
    }

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();

    if extension != "cue" {
        return Err(CommandError::new(
            "invalid_cue_extension",
            "CUE file must have a .cue extension",
        ));
    }

    Ok(path)
}

pub fn validate_mp3_bitrate(
    bitrate: Option<&str>,
    output_format: &str,
) -> CommandResult<Option<String>> {
    if output_format != "mp3" {
        return Ok(None);
    }

    let bitrate = bitrate.unwrap_or("320k").trim().to_ascii_lowercase();

    if !MP3_BITRATES.contains(&bitrate.as_str()) {
        return Err(CommandError::new(
            "invalid_mp3_bitrate",
            "Unsupported MP3 bitrate",
        ));
    }

    Ok(Some(bitrate))
}
