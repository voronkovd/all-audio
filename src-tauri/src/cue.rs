use crate::error::{CommandError, CommandResult};
use crate::path_util::{basename_label, path_file_name_matches};

#[derive(Debug, Clone, PartialEq)]
pub struct CueTrack {
    pub number: u32,
    pub title: Option<String>,
    pub performer: Option<String>,
    pub start_seconds: f64,
}

#[derive(Debug, Default)]
struct TrackBuilder {
    number: u32,
    title: Option<String>,
    performer: Option<String>,
    index01_seconds: Option<f64>,
}

#[derive(Debug, Clone)]
struct CueFileSection {
    file_name: String,
    tracks: Vec<CueTrack>,
}

pub fn parse_cue_file(cue_path: &str, input_path: &str) -> CommandResult<Vec<CueTrack>> {
    let content = std::fs::read_to_string(cue_path).map_err(|error| {
        CommandError::new(
            "cue_read_failed",
            format!("Failed to read CUE file: {error}"),
        )
    })?;

    parse_cue_for_input(&content, input_path)
}

pub fn parse_cue_for_input(content: &str, input_path: &str) -> CommandResult<Vec<CueTrack>> {
    let sections = parse_cue_sections(content)?;
    let tracks = select_tracks_for_input(&sections, input_path)?;
    normalize_cue_tracks(tracks)
}

fn parse_cue_sections(content: &str) -> CommandResult<Vec<CueFileSection>> {
    let mut sections = Vec::new();
    let mut current_file: Option<String> = None;
    let mut current_track: Option<TrackBuilder> = None;
    let mut current_tracks: Vec<CueTrack> = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.to_ascii_uppercase().starts_with("REM ") {
            continue;
        }

        if let Some(file_name) = parse_file_line(line) {
            if let Some(track) = current_track.take() {
                if let Some(parsed) = finalize_track(track) {
                    current_tracks.push(parsed);
                }
            }

            if let Some(file_name) = current_file.take() {
                if !current_tracks.is_empty() {
                    sections.push(CueFileSection {
                        file_name,
                        tracks: std::mem::take(&mut current_tracks),
                    });
                }
            }

            current_file = Some(file_name);
            current_tracks.clear();
            continue;
        }

        if let Some((number, _)) = parse_track_line(line) {
            if let Some(track) = current_track.take() {
                if let Some(parsed) = finalize_track(track) {
                    current_tracks.push(parsed);
                }
            }

            current_track = Some(TrackBuilder {
                number,
                ..TrackBuilder::default()
            });
            continue;
        }

        if let Some(title) = parse_quoted_value(line, "TITLE") {
            if let Some(track) = current_track.as_mut() {
                track.title = Some(title);
            }
            continue;
        }

        if let Some(performer) = parse_quoted_value(line, "PERFORMER") {
            if let Some(track) = current_track.as_mut() {
                track.performer = Some(performer);
            }
            continue;
        }

        if let Some(track) = current_track.as_mut() {
            if let Some(seconds) = parse_index01(line) {
                track.index01_seconds = Some(seconds);
            }
        }
    }

    if let Some(track) = current_track.take() {
        if let Some(parsed) = finalize_track(track) {
            current_tracks.push(parsed);
        }
    }

    if let Some(file_name) = current_file {
        if !current_tracks.is_empty() {
            sections.push(CueFileSection {
                file_name,
                tracks: current_tracks,
            });
        }
    } else if !current_tracks.is_empty() {
        sections.push(CueFileSection {
            file_name: String::new(),
            tracks: current_tracks,
        });
    }

    if sections.is_empty() {
        return Err(CommandError::new(
            "cue_no_tracks",
            "CUE file does not contain any tracks with INDEX 01",
        ));
    }

    Ok(sections)
}

fn select_tracks_for_input(
    sections: &[CueFileSection],
    input_path: &str,
) -> CommandResult<Vec<CueTrack>> {
    let input_name = basename_label(input_path)?;

    let matching: Vec<&CueFileSection> = sections
        .iter()
        .filter(|section| path_file_name_matches(&section.file_name, input_path))
        .collect();

    if matching.len() == 1 {
        return Ok(matching[0].tracks.clone());
    }

    if matching.is_empty() {
        let available = sections
            .iter()
            .map(|section| section.file_name.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        return Err(CommandError::new(
            "cue_file_mismatch",
            format!(
                "CUE file does not contain tracks for \"{input_name}\". Referenced files: {available}"
            ),
        ));
    }

    Err(CommandError::new(
        "cue_ambiguous_file",
        format!("CUE file contains multiple sections for \"{input_name}\""),
    ))
}

pub fn normalize_cue_tracks(mut tracks: Vec<CueTrack>) -> CommandResult<Vec<CueTrack>> {
    tracks.sort_by(|left, right| {
        left.start_seconds
            .partial_cmp(&right.start_seconds)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| left.number.cmp(&right.number))
    });

    for window in tracks.windows(2) {
        let gap = window[1].start_seconds - window[0].start_seconds;
        if gap <= 0.0 {
            return Err(CommandError::new(
                "cue_invalid_track_order",
                format!(
                    "Track {:02} starts at the same time or before track {:02}. Check INDEX 01 values in the CUE file.",
                    window[1].number, window[0].number
                ),
            ));
        }
    }

    Ok(tracks)
}

fn finalize_track(track: TrackBuilder) -> Option<CueTrack> {
    let start_seconds = track.index01_seconds?;

    Some(CueTrack {
        number: track.number,
        title: track.title,
        performer: track.performer,
        start_seconds,
    })
}

fn parse_file_line(line: &str) -> Option<String> {
    if !line.to_ascii_uppercase().starts_with("FILE ") {
        return None;
    }

    parse_quoted_string(line[5..].trim())
}

fn parse_track_line(line: &str) -> Option<(u32, &str)> {
    let upper = line.to_ascii_uppercase();
    if !upper.starts_with("TRACK ") {
        return None;
    }

    let rest = line[6..].trim();
    let mut parts = rest.split_whitespace();
    let number = parts.next()?.parse().ok()?;
    let track_type = parts.next()?;

    if track_type.eq_ignore_ascii_case("AUDIO") {
        Some((number, track_type))
    } else {
        None
    }
}

fn parse_quoted_value(line: &str, key: &str) -> Option<String> {
    let prefix = format!("{key} ");
    if !line
        .to_ascii_uppercase()
        .starts_with(&prefix.to_ascii_uppercase())
    {
        return None;
    }

    let value = line[prefix.len()..].trim();
    parse_quoted_string(value)
}

fn parse_quoted_string(value: &str) -> Option<String> {
    let value = value.trim();
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

fn parse_index01(line: &str) -> Option<f64> {
    let upper = line.to_ascii_uppercase();
    if !upper.starts_with("INDEX 01 ") {
        return None;
    }

    let time = line[9..].trim();
    cue_time_to_seconds(time)
}

pub fn cue_time_to_seconds(time: &str) -> Option<f64> {
    let parts: Vec<&str> = time.trim().split(':').collect();

    let seconds = match parts.len() {
        3 => {
            let minutes: u32 = parts[0].parse().ok()?;
            let seconds: u32 = parts[1].parse().ok()?;
            let frames: u32 = parts[2].parse().ok()?;
            cue_components_to_seconds(0, minutes, seconds, frames)?
        }
        4 => {
            let hours: u32 = parts[0].parse().ok()?;
            let minutes: u32 = parts[1].parse().ok()?;
            let seconds: u32 = parts[2].parse().ok()?;
            let frames: u32 = parts[3].parse().ok()?;
            cue_components_to_seconds(hours, minutes, seconds, frames)?
        }
        _ => return None,
    };

    Some(seconds)
}

fn cue_components_to_seconds(hours: u32, minutes: u32, seconds: u32, frames: u32) -> Option<f64> {
    if frames >= 75 {
        return None;
    }

    Some(
        f64::from(hours) * 3600.0
            + f64::from(minutes) * 60.0
            + f64::from(seconds)
            + f64::from(frames) / 75.0,
    )
}

pub fn track_duration_seconds(current: &CueTrack, next: &CueTrack) -> CommandResult<f64> {
    let duration = next.start_seconds - current.start_seconds;
    if duration <= 0.0 {
        return Err(CommandError::new(
            "cue_invalid_track_duration",
            format!(
                "Track {:02} has an invalid duration relative to track {:02}",
                current.number, next.number
            ),
        ));
    }

    Ok(duration)
}

pub fn track_display_title(track: &CueTrack) -> String {
    track
        .title
        .as_deref()
        .map(str::trim)
        .filter(|title| !title.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| format!("Track {:02}", track.number))
}

pub fn build_track_filename(track: &CueTrack, extension: &str) -> String {
    let title = sanitize_filename_component(&track_display_title(track));
    let title = if title.is_empty() {
        format!("Track {:02}", track.number)
    } else {
        title
    };

    format!("{:02} - {title}.{extension}", track.number)
}

pub fn sanitize_filename_component(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            character => character,
        })
        .collect::<String>()
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TWO_FILE_CUE: &str = r#"
FILE "Сторона_А.flac" WAVE
  TRACK 01 AUDIO
    TITLE "Rap Monkey"
    INDEX 01 00:00:00
  TRACK 05 AUDIO
    TITLE "Русский мороз"
    INDEX 01 17:10:00
FILE "Сторона_Б.flac" WAVE
  TRACK 06 AUDIO
    TITLE "Чужой"
    INDEX 01 00:00:00
  TRACK 09 AUDIO
    TITLE "Бешеный пёс"
    INDEX 01 13:55:50
"#;

    #[test]
    fn parses_basic_cue() {
        let content = r#"
PERFORMER "Album Artist"
TITLE "Album Title"
FILE "album.flac" WAVE
TRACK 01 AUDIO
  TITLE "First Song"
  PERFORMER "Artist A"
  INDEX 01 00:00:00
TRACK 02 AUDIO
  TITLE "Second Song"
  INDEX 01 03:45:50
"#;

        let tracks = parse_cue_for_input(content, "album.flac").unwrap();
        assert_eq!(tracks.len(), 2);
        assert_eq!(tracks[0].title.as_deref(), Some("First Song"));
        assert!((tracks[0].start_seconds - 0.0).abs() < f64::EPSILON);
        assert_eq!(tracks[1].title.as_deref(), Some("Second Song"));
    }

    #[test]
    fn selects_tracks_for_matching_file() {
        let tracks = parse_cue_for_input(TWO_FILE_CUE, "/music/Сторона_А.flac").unwrap();

        assert_eq!(tracks.len(), 2);
        assert_eq!(tracks[0].number, 1);
        assert_eq!(tracks[1].number, 5);
    }

    #[test]
    fn side_b_starts_at_zero_without_conflict() {
        let tracks = parse_cue_for_input(TWO_FILE_CUE, "/music/Сторона_Б.flac").unwrap();

        assert_eq!(tracks.len(), 2);
        assert_eq!(tracks[0].number, 6);
        assert!((tracks[0].start_seconds - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn rejects_mismatched_input_file() {
        let error = parse_cue_for_input(TWO_FILE_CUE, "/music/unknown.flac").unwrap_err();
        assert_eq!(error.code, "cue_file_mismatch");
    }

    #[test]
    fn global_title_does_not_override_track_title() {
        let content = r#"
TITLE "Album"
FILE "album.flac" WAVE
TRACK 01 AUDIO
  INDEX 01 00:00:00
"#;

        let tracks = parse_cue_for_input(content, "album.flac").unwrap();
        assert_eq!(tracks[0].title, None);
        assert_eq!(track_display_title(&tracks[0]), "Track 01");
    }

    #[test]
    fn converts_cue_time() {
        assert!((cue_time_to_seconds("00:00:00").unwrap() - 0.0).abs() < f64::EPSILON);
        assert!((cue_time_to_seconds("01:02:37").unwrap() - (62.0 + 37.0 / 75.0)).abs() < 0.001);
    }

    #[test]
    fn converts_four_part_cue_time() {
        assert!((cue_time_to_seconds("00:15:30:00").unwrap() - (15.0 * 60.0 + 30.0)).abs() < 0.001);
    }

    #[test]
    fn rejects_non_monotonic_tracks() {
        let tracks = vec![
            CueTrack {
                number: 1,
                title: None,
                performer: None,
                start_seconds: 100.0,
            },
            CueTrack {
                number: 2,
                title: None,
                performer: None,
                start_seconds: 100.0,
            },
        ];

        assert!(normalize_cue_tracks(tracks).is_err());
    }

    #[test]
    fn rejects_invalid_frame_values() {
        assert!(cue_time_to_seconds("00:00:75").is_none());
    }
}
