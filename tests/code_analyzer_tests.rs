use catch_cli::code_analyzer::check_rcp_status;
use serde_json::json;

#[tokio::test]
async fn test_check_rcp_status_success() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/session/test-session-id/process")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "process": {
                    "id": "test-session-id",
                    "status": "in-progress",
                    "output": {
                        "docs": [],
                        "fetching-code": {"step": "0/0", "status": "in-progress"},
                        "indexing-code": {"step": "0/0", "status": "not-yet"},
                        "generating-diff": {"files": [], "status": "not-yet"},
                        "generating-docs": {"step": "0/0", "status": "not-yet"},
                        "analyzing-platform": {"status": "not-yet", "platformInfo": {"platform": "", "architectureDescription": ""}},
                        "generating-comment": {"status": "not-yet", "comments": []},
                        "extracting-candidates": {"status": "completed", "candidates": ["candidate1", "candidate2"]},
                        "analyzing-module-structure": {"status": "not-yet", "structure": ""}
                    }
                }
            })
            .to_string(),
        )
        .create_async()
        .await;

    std::env::set_var("CATCH_CLI_BASE_API_URL", server.url());

    let result = check_rcp_status("test-session-id".to_string()).await;

    assert!(result.is_ok());
    let candidates_result = result.unwrap();
    assert_eq!(candidates_result.status, "completed");
    assert_eq!(
        candidates_result.candidates,
        vec!["candidate1", "candidate2"]
    );
}

#[tokio::test]
async fn test_check_rcp_status_no_output() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/session/test-session-id/process")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "process": {}
            })
            .to_string(),
        )
        .create_async()
        .await;

    std::env::set_var("CATCH_CLI_BASE_API_URL", server.url());

    let result = check_rcp_status("test-session-id".to_string()).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_check_rcp_status_api_error() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/session/test-session-id/process")
        .with_status(500)
        .create_async()
        .await;

    std::env::set_var("CATCH_CLI_BASE_API_URL", server.url());

    let result = check_rcp_status("test-session-id".to_string()).await;

    assert!(result.is_err());
}
