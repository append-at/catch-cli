use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use log::{error, warn};

#[derive(Debug)]
pub enum CatchSessionError {
    NoSessionFound,
    MultipleSessionsFound,
    IoError(std::io::Error),
}

impl From<std::io::Error> for CatchSessionError {
    fn from(error: std::io::Error) -> Self {
        CatchSessionError::IoError(error)
    }
}

fn is_session_folder(entry: &fs::DirEntry, regex: &Regex) -> Option<(String, PathBuf)> {
    let file_name = entry.file_name();
    file_name.to_str().and_then(|name| {
        regex.captures(name).and_then(|captures| {
            captures
                .get(1)
                .map(|session_id| (session_id.as_str().to_string(), entry.path()))
        })
    })
}

pub fn handle_sessions(temp_path: &Path) -> Result<String, CatchSessionError> {
    let regex = Regex::new(r"^catch_session_(.+)$").unwrap();

    let sessions: Vec<(String, PathBuf)> = fs::read_dir(temp_path)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| is_session_folder(&entry, &regex))
        .collect();

    match sessions.len() {
        0 => {
            warn!("No active sessions found. Please start a new session.");
            Err(CatchSessionError::NoSessionFound)
        }
        1 => Ok(sessions[0].0.clone()),
        _ => {
            for (_, folder) in sessions {
                fs::remove_dir_all(folder)?;
            }
            error!("Multiple active sessions detected on this device. All sessions have been terminated. Please restart catch-cli.");
            Err(CatchSessionError::MultipleSessionsFound)
        }
    }
}
