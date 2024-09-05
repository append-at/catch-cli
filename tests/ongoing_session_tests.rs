use catch_cli::ongoing_session::{handle_sessions, CatchSessionError};
use std::path::PathBuf;
use std::fs;
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
