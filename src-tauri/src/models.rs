use serde::{Deserialize, Serialize};

use crate::metadata::Mp3Tags;

#[derive(Debug, Serialize)]
pub struct FfmpegAvailability {
    pub ffmpeg: bool,
    pub ffprobe: bool,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct AudioMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub year: Option<String>,
    pub genre: Option<String>,
    pub track: Option<String>,
}

impl From<Mp3Tags> for AudioMetadata {
    fn from(tags: Mp3Tags) -> Self {
        Self {
            title: tags.title,
            artist: tags.artist,
            album: tags.album,
            album_artist: tags.album_artist,
            year: tags.year,
            genre: tags.genre,
            track: tags.track,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AudioProbe {
    pub duration: Option<f64>,
    pub codec: Option<String>,
    pub sample_rate: Option<u32>,
    pub channels: Option<u32>,
    pub bitrate: Option<u64>,
    pub metadata: AudioMetadata,
}

#[derive(Debug, Deserialize, Default, Clone)]
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
    #[serde(default)]
    pub mp3_tags: Option<Mp3Tags>,
    #[serde(default)]
    pub cover_path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ConvertAudioResult {
    pub output_paths: Vec<String>,
}
