pub const BASE_CATCH_API_URL: &str = "https://api.dev.trycatch.ai";

use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Deserialize;

pub mod cli_entity;
pub mod request_entity;
pub mod session_status_entity;

#[derive(Debug)]
pub enum CatchApiError {
    RequestFailed(reqwest::Error),
    ResponseParseError(reqwest::Error),
    InvalidResponse,
}

pub struct CatchApiClient {
    client: Client,
    base_url: String,
}

pub enum CatchApiResponse<T: for<'de> Deserialize<'de>> {
    Success(T),
    NoContent,
}

impl From<reqwest::Error> for CatchApiError {
    fn from(error: reqwest::Error) -> Self {
        CatchApiError::RequestFailed(error)
    }
}

impl Default for CatchApiClient {
    fn default() -> Self {
        let base_url = std::env::var("CATCH_CLI_BASE_API_URL");

        Self {
            client: Client::new(),
            base_url: base_url.unwrap_or_else(|_| BASE_CATCH_API_URL.to_string()),
        }
    }
}

impl CatchApiClient {
    async fn handle_response<T: DeserializeOwned>(
        response: Response,
    ) -> Result<CatchApiResponse<T>, CatchApiError> {
        match response.error_for_status() {
            Ok(res) => match StatusCode::NO_CONTENT == res.status() {
                true => Ok(CatchApiResponse::NoContent),
                false => Ok(CatchApiResponse::Success(match res.json().await {
                    Ok(json) => json,
                    Err(e) => return Err(CatchApiError::ResponseParseError(e)),
                })),
            },
            Err(e) => Err(CatchApiError::RequestFailed(e)),
        }
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<CatchApiResponse<T>, CatchApiError> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await?;
        Self::handle_response(response).await
    }

    pub async fn post<T: DeserializeOwned, U: serde::Serialize + ?Sized>(
        &self,
        endpoint: &str,
        body: &U,
    ) -> Result<CatchApiResponse<T>, CatchApiError> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.post(&url).json(body).send().await?;
        Self::handle_response(response).await
    }

    pub async fn put<T: DeserializeOwned, U: serde::Serialize + ?Sized>(
        &self,
        endpoint: &str,
        body: &U,
    ) -> Result<CatchApiResponse<T>, CatchApiError> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.put(&url).json(body).send().await?;
        Self::handle_response(response).await
    }

    pub async fn delete<T: DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<CatchApiResponse<T>, CatchApiError> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.delete(&url).send().await?;
        Self::handle_response(response).await
    }
}
