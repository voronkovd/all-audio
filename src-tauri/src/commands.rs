use crate::error::CommandResult;
use crate::ffmpeg::{
    check_ffmpeg_available as detect_ffmpeg_availability, convert_audio_blocking,
    probe_audio_file as probe_audio,
};
use crate::ffmpeg_provider::FfmpegProvider;
use crate::metadata::{parse_cue_album_info, CueAlbumInfo};
use crate::models::{AudioProbe, ConvertAudioRequest, ConvertAudioResult, FfmpegAvailability};

#[tauri::command]
pub fn check_ffmpeg_available(provider: tauri::State<'_, FfmpegProvider>) -> FfmpegAvailability {
    detect_ffmpeg_availability(&provider)
}

#[tauri::command]
pub fn probe_audio_file(
    provider: tauri::State<'_, FfmpegProvider>,
    input_path: String,
) -> CommandResult<AudioProbe> {
    probe_audio(&provider, &input_path)
}

#[tauri::command]
pub fn read_cue_album_info(cue_path: String) -> CommandResult<CueAlbumInfo> {
    let content = std::fs::read_to_string(&cue_path).map_err(|error| {
        crate::error::CommandError::new(
            "cue_read_failed",
            format!("Failed to read CUE file: {error}"),
        )
    })?;

    Ok(parse_cue_album_info(&content))
}

#[tauri::command]
pub async fn convert_audio(
    provider: tauri::State<'_, FfmpegProvider>,
    request: ConvertAudioRequest,
) -> CommandResult<ConvertAudioResult> {
    let tools = provider.inner().clone();

    tauri::async_runtime::spawn_blocking(move || convert_audio_blocking(&tools, &request))
        .await
        .map_err(|error| {
            crate::error::CommandError::new(
                "conversion_task_failed",
                format!("Conversion task failed: {error}"),
            )
        })?
}
