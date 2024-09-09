use catch_cli::api_client::{CatchApiClient, CatchApiResponse};
use mockito::Server;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestUser {
    id: u64,
    name: String,
}

#[tokio::test]
async fn test_get_request() {
    let mut server = Server::new_async().await;
    let mock_response = TestUser {
        id: 1,
        name: "Dora Lee".to_string(),
    };

    let mock = server
        .mock("GET", "/users/1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&mock_response).unwrap())
        .create_async()
        .await;

    std::env::set_var("CATCH_CLI_BASE_API_URL", server.url());

    let client = CatchApiClient::new();
    let response = client.get::<TestUser>("/users/1").await.unwrap();

    mock.assert();

    match response {
        CatchApiResponse::Success(user) => assert_eq!(user, mock_response),
        _ => panic!("Expected Success response"),
    }
}

#[tokio::test]
async fn test_post_request() {
    let mut server = Server::new_async().await;
    let new_user = TestUser {
        id: 0,
        name: "Dora Lee".to_string(),
    };
    let created_user = TestUser {
        id: 2,
        name: "Dora Dora Lee".to_string(),
    };

    let mock = server
        .mock("POST", "/users")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&created_user).unwrap())
        .match_body(mockito::Matcher::Json(
            serde_json::to_value(&new_user).unwrap(),
        ))
        .create_async()
        .await;

    std::env::set_var("CATCH_CLI_BASE_API_URL", server.url());

    let client = CatchApiClient::new();
    let response = client
        .post::<TestUser, _>("/users", &new_user)
        .await
        .unwrap();

    mock.assert();

    match response {
        CatchApiResponse::Success(user) => assert_eq!(user, created_user),
        _ => panic!("Expected Success response"),
    }
}

#[tokio::test]
async fn test_put_request() {
    let mut server = Server::new_async().await;
    let update_user = TestUser {
        id: 1,
        name: "Dora Updated".to_string(),
    };

    let mock = server
        .mock("PUT", "/users/1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&update_user).unwrap())
        .match_body(mockito::Matcher::Json(
            serde_json::to_value(&update_user).unwrap(),
        ))
        .create_async()
        .await;

    std::env::set_var("CATCH_CLI_BASE_API_URL", server.url());

    let client = CatchApiClient::new();
    let response = client
        .put::<TestUser, _>("/users/1", &update_user)
        .await
        .unwrap();

    mock.assert();

    match response {
        CatchApiResponse::Success(user) => assert_eq!(user, update_user),
        _ => panic!("Expected Success response"),
    }
}

#[tokio::test]
async fn test_delete_request() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("DELETE", "/users/1")
        .with_status(204)
        .create_async()
        .await;

    std::env::set_var("CATCH_CLI_BASE_API_URL", server.url());

    let client = CatchApiClient::new();
    let response = client.delete::<()>("/users/1").await.unwrap();

    mock.assert();

    match response {
        CatchApiResponse::NoContent => {}
        _ => panic!("Expected NoContent response"),
    }
}

#[tokio::test]
async fn test_no_content_response() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/empty")
        .with_status(204)
        .create_async()
        .await;

    std::env::set_var("CATCH_CLI_BASE_API_URL", server.url());

    let client = CatchApiClient::new();
    let response = client.get::<()>("/empty").await.unwrap();

    mock.assert();

    match response {
        CatchApiResponse::NoContent => {}
        _ => panic!("Expected NoContent response"),
    }
}
