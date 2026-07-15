use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

use crate::cue::{build_track_filename, parse_cue_file, track_duration_seconds, CueTrack};
use crate::error::{CommandError, CommandResult};
use crate::ffmpeg_provider::FfmpegProvider;
use crate::metadata::{
    append_mp3_metadata, parse_cue_album_info, parse_ffprobe_tags, validate_cover_image, Mp3Tags,
};
use crate::models::{
    AudioMetadata, AudioProbe, ConvertAudioRequest, ConvertAudioResult, FfmpegAvailability,
};
use crate::validation::{
    build_output_path, is_flac_file, validate_cue_file, validate_input_file, validate_mp3_bitrate,
    validate_output_dir, validate_output_filename, validate_output_format,
};

struct SegmentOptions {
    output_format: String,
    mp3_bitrate: Option<String>,
    tags: Option<Mp3Tags>,
    cover_path: Option<PathBuf>,
}

fn run_command(mut command: Command) -> CommandResult<std::process::Output> {
    command.output().map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            CommandError::new(
                "command_not_found",
                "Bundled FFmpeg tool could not be executed",
            )
        } else {
            CommandError::new(
                "command_failed",
                format!("Failed to run external command: {error}"),
            )
        }
    })
}

pub fn check_ffmpeg_available(provider: &FfmpegProvider) -> FfmpegAvailability {
    provider.check_availability()
}

#[derive(Debug, Deserialize)]
struct FfprobeOutput {
    streams: Option<Vec<FfprobeStream>>,
    format: Option<FfprobeFormat>,
}

#[derive(Debug, Deserialize)]
struct FfprobeStream {
    codec_type: Option<String>,
    codec_name: Option<String>,
    sample_rate: Option<String>,
    channels: Option<u32>,
    bit_rate: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FfprobeFormat {
    duration: Option<String>,
    bit_rate: Option<String>,
    tags: Option<std::collections::HashMap<String, String>>,
}

pub fn probe_audio_file(provider: &FfmpegProvider, input_path: &str) -> CommandResult<AudioProbe> {
    let path = validate_input_file(input_path)?;

    let mut command = Command::new(provider.get_ffprobe_path());
    command
        .arg("-v")
        .arg("error")
        .arg("-select_streams")
        .arg("a:0")
        .arg("-show_entries")
        .arg("stream=codec_name,sample_rate,channels,bit_rate")
        .arg("-show_entries")
        .arg("format=duration,bit_rate:tags")
        .arg("-of")
        .arg("json")
        .arg(&path);

    let output = run_command(command)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(CommandError::new(
            "ffprobe_failed",
            if stderr.trim().is_empty() {
                "FFprobe failed to analyze the input file".to_string()
            } else {
                stderr.trim().to_string()
            },
        ));
    }

    let parsed: FfprobeOutput = serde_json::from_slice(&output.stdout)
        .map_err(|_| CommandError::new("ffprobe_parse_failed", "Failed to parse FFprobe output"))?;

    let audio_stream = parsed
        .streams
        .as_ref()
        .and_then(|streams| {
            streams
                .iter()
                .find(|stream| stream.codec_type.as_deref() == Some("audio"))
        })
        .or_else(|| parsed.streams.as_ref().and_then(|streams| streams.first()));

    let duration = parsed
        .format
        .as_ref()
        .and_then(|format| format.duration.as_ref())
        .and_then(|value| value.parse::<f64>().ok());

    let codec = audio_stream.and_then(|stream| stream.codec_name.clone());

    let sample_rate = audio_stream
        .and_then(|stream| stream.sample_rate.as_ref())
        .and_then(|value| value.parse::<u32>().ok());

    let channels = audio_stream.and_then(|stream| stream.channels);

    let bitrate = audio_stream
        .and_then(|stream| stream.bit_rate.as_ref())
        .and_then(|value| value.parse::<u64>().ok())
        .or_else(|| {
            parsed
                .format
                .as_ref()
                .and_then(|format| format.bit_rate.as_ref())
                .and_then(|value| value.parse::<u64>().ok())
        });

    let metadata = parsed
        .format
        .as_ref()
        .and_then(|format| format.tags.as_ref())
        .map(parse_ffprobe_tags)
        .map(AudioMetadata::from)
        .unwrap_or_default();

    Ok(AudioProbe {
        duration,
        codec,
        sample_rate,
        channels,
        bitrate,
        metadata,
    })
}

pub fn convert_audio_blocking(
    provider: &FfmpegProvider,
    request: &ConvertAudioRequest,
) -> CommandResult<ConvertAudioResult> {
    if request.split_by_cue.unwrap_or(false) {
        return split_flac_by_cue(provider, request);
    }

    let output_path = convert_single_file(provider, request)?;
    Ok(ConvertAudioResult {
        output_paths: vec![output_path],
    })
}

fn resolve_segment_options(request: &ConvertAudioRequest) -> CommandResult<SegmentOptions> {
    let output_format = validate_output_format(&request.output_format)?;
    let mp3_bitrate = validate_mp3_bitrate(request.mp3_bitrate.as_deref(), &output_format)?;

    let cover_path = match request.cover_path.as_deref() {
        Some(path) if !path.trim().is_empty() => Some(validate_cover_image(path)?),
        _ => None,
    };

    Ok(SegmentOptions {
        output_format,
        mp3_bitrate,
        tags: request.mp3_tags.clone(),
        cover_path,
    })
}

fn convert_single_file(
    provider: &FfmpegProvider,
    request: &ConvertAudioRequest,
) -> CommandResult<String> {
    let input = validate_input_file(&request.input_path)?;
    let output_dir = validate_output_dir(&request.output_dir)?;
    let output_filename = validate_output_filename(&request.output_filename)?;
    let options = resolve_segment_options(request)?;
    let output_path = build_output_path(&output_dir, &output_filename, &options.output_format)?;

    if paths_are_same_file(&input, &output_path) {
        return Err(CommandError::new(
            "same_input_output",
            "Input and output files must be different",
        ));
    }

    run_ffmpeg_segment(provider, &input, &output_path, 0.0, None, &options)?;

    Ok(output_path.to_string_lossy().into_owned())
}

fn split_flac_by_cue(
    provider: &FfmpegProvider,
    request: &ConvertAudioRequest,
) -> CommandResult<ConvertAudioResult> {
    let input = validate_input_file(&request.input_path)?;

    if !is_flac_file(&input) {
        return Err(CommandError::new(
            "cue_requires_flac",
            "CUE splitting is only supported for FLAC input files",
        ));
    }

    let cue_path = request
        .cue_path
        .as_deref()
        .ok_or_else(|| CommandError::new("cue_path_required", "CUE file path is required"))?;

    validate_cue_file(cue_path)?;
    let output_dir = validate_output_dir(&request.output_dir)?;
    let base_options = resolve_segment_options(request)?;

    let cue_content = std::fs::read_to_string(cue_path).map_err(|error| {
        CommandError::new(
            "cue_read_failed",
            format!("Failed to read CUE file: {error}"),
        )
    })?;
    let cue_album = parse_cue_album_info(&cue_content);

    let input_path = request.input_path.as_str();
    let tracks = parse_cue_file(cue_path, input_path)?;
    let output_paths = build_split_output_paths(&output_dir, &tracks, &base_options.output_format)?;

    let base_tags = request.mp3_tags.clone().unwrap_or_default();
    let mut created_paths = Vec::with_capacity(output_paths.len());

    for (index, (track, output_path)) in tracks.iter().zip(output_paths.iter()).enumerate() {
        let duration = tracks
            .get(index + 1)
            .map(|next_track| track_duration_seconds(track, next_track))
            .transpose()?;

        let track_tags = if base_options.output_format == "mp3" {
            Some(base_tags.for_cue_track(
                &cue_album,
                track.title.as_deref(),
                track.performer.as_deref(),
                track.number,
            ))
        } else {
            None
        };

        let options = SegmentOptions {
            output_format: base_options.output_format.clone(),
            mp3_bitrate: base_options.mp3_bitrate.clone(),
            tags: track_tags,
            cover_path: base_options.cover_path.clone(),
        };

        if let Err(error) = run_ffmpeg_segment(
            provider,
            &input,
            output_path,
            track.start_seconds,
            duration,
            &options,
        ) {
            remove_created_files(&created_paths);
            return Err(error);
        }

        created_paths.push(output_path.clone());
    }

    Ok(ConvertAudioResult {
        output_paths: created_paths
            .iter()
            .map(|path| path.to_string_lossy().into_owned())
            .collect(),
    })
}

fn remove_created_files(paths: &[PathBuf]) {
    for path in paths {
        let _ = std::fs::remove_file(path);
    }
}

fn paths_are_same_file(left: &Path, right: &Path) -> bool {
    match (left.canonicalize(), right.canonicalize()) {
        (Ok(left), Ok(right)) => left == right,
        _ => left == right,
    }
}

fn build_split_output_paths(
    output_dir: &Path,
    tracks: &[CueTrack],
    output_format: &str,
) -> CommandResult<Vec<PathBuf>> {
    let mut paths = Vec::with_capacity(tracks.len());

    for track in tracks {
        let filename = build_track_filename(track, output_format);
        validate_output_filename(&filename)?;
        let output_path = output_dir.join(&filename);

        if output_path.exists() {
            return Err(CommandError::new(
                "output_file_exists",
                format!("Output file already exists: {}", output_path.display()),
            ));
        }

        paths.push(output_path);
    }

    Ok(paths)
}

fn format_ffmpeg_time(seconds: f64) -> String {
    format!("{seconds:.6}")
}

fn run_ffmpeg_segment(
    provider: &FfmpegProvider,
    input: &Path,
    output: &Path,
    start_seconds: f64,
    duration_seconds: Option<f64>,
    options: &SegmentOptions,
) -> CommandResult<()> {
    let embed_cover = options.output_format == "mp3" && options.cover_path.is_some();

    let mut command = Command::new(provider.get_ffmpeg_path());
    command.arg("-hide_banner").arg("-loglevel").arg("error");

    if start_seconds > 0.0 {
        command.arg("-ss").arg(format_ffmpeg_time(start_seconds));
    }

    command.arg("-i").arg(input);

    if let Some(cover) = options.cover_path.as_deref() {
        command.arg("-i").arg(cover);
    }

    if let Some(duration) = duration_seconds {
        command.arg("-t").arg(format_ffmpeg_time(duration));
    }

    if options.output_format == "mp3" {
        command.arg("-map").arg("0:a:0");

        if embed_cover {
            command.arg("-map").arg("1");
            command.arg("-c:v").arg("copy");
            command.arg("-disposition:v:0").arg("attached_pic");
            command.arg("-metadata:s:v:0").arg("title=Album cover");
            command.arg("-metadata:s:v:0").arg("comment=Cover (front)");
        } else {
            command.arg("-vn");
        }

        command.arg("-c:a").arg("libmp3lame");

        if let Some(bitrate) = options.mp3_bitrate.as_deref() {
            command.arg("-b:a").arg(bitrate);
        }

        command.arg("-id3v2_version").arg("3");

        if let Some(tags) = options.tags.as_ref().filter(|tags| !tags.is_empty()) {
            append_mp3_metadata(&mut command, tags);
        }
    } else {
        command.arg("-vn");

        if let Some(bitrate) = options.mp3_bitrate.as_deref() {
            command.arg("-b:a").arg(bitrate);
        }
    }

    command.arg(output);

    let output = run_command(command)?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    Err(CommandError::new(
        "ffmpeg_failed",
        if stderr.trim().is_empty() {
            "FFmpeg conversion failed".to_string()
        } else {
            stderr.trim().to_string()
        },
    ))
}
