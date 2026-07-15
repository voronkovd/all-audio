use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{CommandError, CommandResult};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Mp3Tags {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub year: Option<String>,
    pub genre: Option<String>,
    pub track: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct CueAlbumInfo {
    pub title: Option<String>,
    pub performer: Option<String>,
}

impl Mp3Tags {
    pub fn is_empty(&self) -> bool {
        self.title.as_ref().is_none_or(|v| v.trim().is_empty())
            && self.artist.as_ref().is_none_or(|v| v.trim().is_empty())
            && self.album.as_ref().is_none_or(|v| v.trim().is_empty())
            && self
                .album_artist
                .as_ref()
                .is_none_or(|v| v.trim().is_empty())
            && self.year.as_ref().is_none_or(|v| v.trim().is_empty())
            && self.genre.as_ref().is_none_or(|v| v.trim().is_empty())
            && self.track.as_ref().is_none_or(|v| v.trim().is_empty())
    }

    pub fn for_cue_track(
        &self,
        album: &CueAlbumInfo,
        track_title: Option<&str>,
        track_performer: Option<&str>,
        track_number: u32,
    ) -> Mp3Tags {
        let mut tags = self.clone();

        if tags.title.as_ref().is_none_or(|v| v.trim().is_empty()) {
            tags.title = track_title.map(str::to_string);
        }

        if tags.artist.as_ref().is_none_or(|v| v.trim().is_empty()) {
            tags.artist = track_performer
                .map(str::to_string)
                .or_else(|| album.performer.clone());
        }

        if tags.album.as_ref().is_none_or(|v| v.trim().is_empty()) {
            tags.album = album.title.clone();
        }

        if tags
            .album_artist
            .as_ref()
            .is_none_or(|v| v.trim().is_empty())
        {
            tags.album_artist = album.performer.clone();
        }

        if tags.track.as_ref().is_none_or(|v| v.trim().is_empty()) {
            tags.track = Some(track_number.to_string());
        }

        tags
    }
}

pub fn parse_ffprobe_tags(tags: &HashMap<String, String>) -> Mp3Tags {
    Mp3Tags {
        title: tag_value(tags, &["title", "TITLE"]),
        artist: tag_value(tags, &["artist", "ARTIST"]),
        album: tag_value(tags, &["album", "ALBUM"]),
        album_artist: tag_value(
            tags,
            &["album_artist", "albumartist", "ALBUMARTIST", "TPE2"],
        ),
        year: tag_value(tags, &["date", "year", "DATE", "TYER", "TDRC"]),
        genre: tag_value(tags, &["genre", "GENRE"]),
        track: tag_value(tags, &["track", "tracknumber", "TRCK", "TRACKNUMBER"]),
    }
}

fn tag_value(tags: &HashMap<String, String>, keys: &[&str]) -> Option<String> {
    for key in keys {
        if let Some(value) = tags
            .get(*key)
            .or_else(|| tags.get(&key.to_ascii_lowercase()))
        {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
        for (tag_key, tag_value) in tags {
            if tag_key.eq_ignore_ascii_case(key) {
                let trimmed = tag_value.trim();
                if !trimmed.is_empty() {
                    return Some(trimmed.to_string());
                }
            }
        }
    }
    None
}

pub fn parse_cue_album_info(content: &str) -> CueAlbumInfo {
    let mut album = CueAlbumInfo::default();
    let mut in_track = false;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.to_ascii_uppercase().starts_with("REM ") {
            continue;
        }

        if line.to_ascii_uppercase().starts_with("FILE ") {
            break;
        }

        if line.to_ascii_uppercase().starts_with("TRACK ") {
            in_track = true;
            continue;
        }

        if in_track {
            continue;
        }

        if let Some(title) = parse_quoted_cue_value(line, "TITLE") {
            album.title = Some(title);
        }

        if let Some(performer) = parse_quoted_cue_value(line, "PERFORMER") {
            album.performer = Some(performer);
        }
    }

    album
}

fn parse_quoted_cue_value(line: &str, key: &str) -> Option<String> {
    let prefix = format!("{key} ");
    if !line
        .to_ascii_uppercase()
        .starts_with(&prefix.to_ascii_uppercase())
    {
        return None;
    }

    let value = line[prefix.len()..].trim();
    if let Some(quoted) = value.strip_prefix('"') {
        let end = quoted.find('"')?;
        return Some(quoted[..end].to_string());
    }

    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

pub fn append_mp3_metadata(command: &mut std::process::Command, tags: &Mp3Tags) {
    append_metadata_pair(command, "title", tags.title.as_deref());
    append_metadata_pair(command, "artist", tags.artist.as_deref());
    append_metadata_pair(command, "album", tags.album.as_deref());
    append_metadata_pair(command, "album_artist", tags.album_artist.as_deref());
    append_metadata_pair(command, "date", tags.year.as_deref());
    append_metadata_pair(command, "genre", tags.genre.as_deref());
    append_metadata_pair(command, "track", tags.track.as_deref());
}

fn append_metadata_pair(command: &mut std::process::Command, key: &str, value: Option<&str>) {
    if let Some(value) = value.filter(|v| !v.trim().is_empty()) {
        command.arg("-metadata").arg(format!("{key}={value}"));
    }
}

const COVER_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp", "bmp"];

pub fn validate_cover_image(path: &str) -> CommandResult<std::path::PathBuf> {
    let path = Path::new(path.trim());

    if path.as_os_str().is_empty() {
        return Err(CommandError::new(
            "invalid_cover_path",
            "Cover image path is empty",
        ));
    }

    if !path.exists() {
        return Err(CommandError::new(
            "cover_not_found",
            "Cover image file does not exist",
        ));
    }

    if !path.is_file() {
        return Err(CommandError::new(
            "invalid_cover_path",
            "Cover path is not a file",
        ));
    }

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();

    if !COVER_EXTENSIONS.contains(&extension.as_str()) {
        return Err(CommandError::new(
            "unsupported_cover_extension",
            "Cover image must be JPG, PNG, WebP, or BMP",
        ));
    }

    Ok(path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_cue_album_metadata() {
        let content = r#"
PERFORMER "Demo Artist"
TITLE "Demo Album"
FILE "album.flac" WAVE
  TRACK 01 AUDIO
    TITLE "Intro"
"#;

        let album = parse_cue_album_info(content);
        assert_eq!(album.title.as_deref(), Some("Demo Album"));
        assert_eq!(album.performer.as_deref(), Some("Demo Artist"));
    }

    #[test]
    fn merges_track_tags_from_cue() {
        let base = Mp3Tags {
            album: Some("My Album".to_string()),
            ..Default::default()
        };
        let album = CueAlbumInfo {
            title: Some("CUE Album".to_string()),
            performer: Some("CUE Artist".to_string()),
        };

        let tags = base.for_cue_track(&album, Some("Track One"), Some("Guest"), 2);
        assert_eq!(tags.title.as_deref(), Some("Track One"));
        assert_eq!(tags.artist.as_deref(), Some("Guest"));
        assert_eq!(tags.album.as_deref(), Some("My Album"));
        assert_eq!(tags.track.as_deref(), Some("2"));
    }
}
