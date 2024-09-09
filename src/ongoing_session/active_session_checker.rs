use crate::api_client::session_status_entity::CatchSessionStatusResponse;
use crate::api_client::{CatchApiClient, CatchApiError, CatchApiResponse, BASE_CATCH_API_URL};
use log::{error, warn};
use regex::Regex;
use reqwest::StatusCode;
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Debug)]
pub enum CatchSessionError {
    NoSessionFound,
    MultipleSessionsFound,
    IoError(io::Error),
}

impl From<io::Error> for CatchSessionError {
    fn from(error: io::Error) -> Self {
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

fn get_session_folders(temp_path: &Path) -> io::Result<Vec<(String, PathBuf)>> {
    let regex = Regex::new(r"^catch_session_(.+)$").unwrap();

    let sessions: Vec<(String, PathBuf)> = fs::read_dir(temp_path)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| is_session_folder(&entry, &regex))
        .collect();

    Ok(sessions)
}

pub async fn is_session_valid(session_id: String) -> io::Result<bool> {
    let url = std::env::var("CATCH_CLI_BASE_API_URL");
    let api_client = match url {
        Ok(url) => CatchApiClient::new(url.as_str()),
        Err(_) => CatchApiClient::new(BASE_CATCH_API_URL),
    };

    let response = api_client
        .get::<CatchSessionStatusResponse>(format!("/session/{}/process", session_id).as_str())
        .await;

    match response {
        Ok(rst) => match rst {
            CatchApiResponse::Success(response) => {
                if response.process.output.is_none()
                    && response.process.status.is_none()
                    && response.process.id.is_none()
                {
                    Ok(true)
                } else {
                    let temp_path = std::env::temp_dir();
                    let sessions = get_session_folders(&temp_path)?;
                    for (_, folder) in sessions {
                        fs::remove_dir_all(folder)?;
                    }

                    Ok(false)
                }
            }
            CatchApiResponse::NoContent => {
                error!("API request failed");
                Err(io::Error::new(io::ErrorKind::Other, "API request failed"))
            }
        },
        Err(e) => match e {
            CatchApiError::RequestFailed(e) => {
                let status_code = e.status();
                if status_code == Some(StatusCode::NOT_FOUND) {
                    error!("Session not found");
                    Err(io::Error::new(io::ErrorKind::Other, e))
                } else {
                    error!("API request failed");
                    Err(io::Error::new(io::ErrorKind::Other, e))
                }
            }
            CatchApiError::ResponseParseError(e) => {
                error!("API response parse error: {}", e);
                Err(io::Error::new(io::ErrorKind::Other, e))
            }
        },
    }
}

pub fn handle_sessions(temp_path: &Path) -> Result<String, CatchSessionError> {
    let sessions = get_session_folders(temp_path)?;

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
