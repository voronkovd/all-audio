use std::path::Path;

use crate::error::{CommandError, CommandResult};

pub fn path_file_name_matches(cue_file: &str, input_path: &str) -> bool {
    let cue_name = Path::new(cue_file).file_name();
    let input_name = Path::new(input_path).file_name();

    match (cue_name, input_name) {
        (Some(left), Some(right)) if left == right => true,
        (Some(left), Some(right)) => match (left.to_str(), right.to_str()) {
            (Some(left), Some(right)) => left.eq_ignore_ascii_case(right),
            _ => false,
        },
        _ => false,
    }
}

pub fn basename_label(path: &str) -> CommandResult<String> {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .map(str::to_string)
        .ok_or_else(|| {
            CommandError::new(
                "invalid_input_path",
                "Could not determine the file name from the path",
            )
        })
}
