use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct FfmpegAvailability {
    pub ffmpeg: bool,
    pub ffprobe: bool,
}

#[derive(Debug, Serialize)]
pub struct AudioProbe {
    pub duration: Option<f64>,
    pub codec: Option<String>,
    pub sample_rate: Option<u32>,
    pub channels: Option<u32>,
    pub bitrate: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ConvertAudioRequest {
    pub input_path: String,
    pub output_dir: String,
    pub output_filename: String,
    pub output_format: String,
    #[serde(default)]
    pub mp3_bitrate: Option<String>,
    #[serde(default)]
    pub cue_path: Option<String>,
    #[serde(default)]
    pub split_by_cue: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ConvertAudioResult {
    pub output_paths: Vec<String>,
}
