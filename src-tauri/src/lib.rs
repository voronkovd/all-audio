mod commands;
mod cue;
mod error;
mod ffmpeg;
mod ffmpeg_provider;
mod models;
mod path_util;
mod validation;

use commands::{check_ffmpeg_available, convert_audio, probe_audio_file};
use ffmpeg_provider::FfmpegProvider;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let provider = FfmpegProvider::initialize(app.handle())?;
            app.manage(provider);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            check_ffmpeg_available,
            probe_audio_file,
            convert_audio
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
