use catch_cli::ongoing_session::active_session_checker::{
    handle_sessions, is_session_valid, CatchSessionError,
};
use serde_json::json;
use std::path::PathBuf;
use std::{fs, io};
use tempfile::{tempdir, TempDir};

fn setup_temp_session(session_id: &str) -> (TempDir, PathBuf) {
    let temp_dir = tempdir().unwrap();
    let session_path = temp_dir
        .path()
        .join(format!("catch_session_{}", session_id));
    fs::create_dir(&session_path).unwrap();
    (temp_dir, session_path)
}

#[test]
fn test_no_sessions() {
    let temp_dir = tempdir().unwrap();
    let result = handle_sessions(temp_dir.path());
    assert!(matches!(result, Err(CatchSessionError::NoSessionFound)));
}

#[test]
fn test_single_session() {
    let (temp_dir, _) = setup_temp_session("123");
    let result = handle_sessions(temp_dir.path());
    assert!(matches!(result, Ok(session_id) if session_id == "123"));
}

#[test]
fn test_multiple_sessions() {
    let temp_dir = tempdir().unwrap();
    let session_path1 = temp_dir.path().join("catch_session_123");
    let session_path2 = temp_dir.path().join("catch_session_456");
    fs::create_dir(&session_path1).unwrap();
    fs::create_dir(&session_path2).unwrap();

    let result = handle_sessions(temp_dir.path());
    assert!(matches!(
        result,
        Err(CatchSessionError::MultipleSessionsFound)
    ));

    // Check if the session folders were deleted
    assert!(!session_path1.exists());
    assert!(!session_path2.exists());
}

#[test]
fn test_ignore_non_session_folders() {
    let (temp_dir, _) = setup_temp_session("123");
    let non_session_dir = temp_dir.path().join("not_a_session");
    fs::create_dir(&non_session_dir).unwrap();

    let result = handle_sessions(temp_dir.path());
    assert!(matches!(result, Ok(session_id) if session_id == "123"));

    // Check if the non-session folder still exists
    assert!(non_session_dir.exists());
}

#[tokio::test]
async fn test_valid_session() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/session/valid_session/process")
        .with_status(200)
        .with_body(
            json!({
                "process": {
                    "output": null,
                    "status": null,
                    "id": null
                }
            })
            .to_string(),
        )
        .create_async()
        .await;

    std::env::set_var("CATCH_CLI_BASE_API_URL", server.url());

    let result = is_session_valid("valid_session".to_string()).await;
    mock.assert();
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_invalid_session() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/session/invalid_session/process")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "process": {
                    "status": "completed",
                    "id": "123"
                }
            })
            .to_string(),
        )
        .create_async()
        .await;

    std::env::set_var("CATCH_CLI_BASE_API_URL", server.url());

    let result = is_session_valid("invalid_session".to_string()).await;
    mock.assert();
    println!("{:?}", result);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[tokio::test]
async fn test_request_failed_404() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/session/not_found/process")
        .with_status(404)
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    std::env::set_var("CATCH_CLI_BASE_API_URL", server.url());

    let result = is_session_valid("not_found".to_string()).await;
    mock.assert();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Other);
}

#[tokio::test]
async fn test_request_failed_other() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/session/server_error/process")
        .with_header("content-type", "application/json")
        .with_status(500)
        .create_async()
        .await;

    std::env::set_var("CATCH_CLI_BASE_API_URL", server.url());

    let result = is_session_valid("server_error".to_string()).await;
    mock.assert();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Other);
}

#[tokio::test]
async fn test_response_parse_error() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/session/parse_error/process")
        .with_status(200)
        .with_body("Invalid JSON")
        .create_async()
        .await;

    std::env::set_var("CATCH_CLI_BASE_API_URL", server.url());

    let result = is_session_valid("parse_error".to_string()).await;
    mock.assert();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Other);
}
