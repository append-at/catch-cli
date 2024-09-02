pub const BASE_CATCH_API_URL: &str = "https://api.dev.trycatch.ai";

use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Deserialize;

pub mod cli_entity;
pub mod request_entity;
pub mod session_entity;

#[derive(Debug)]
pub enum CatchApiError {
    RequestFailed(reqwest::Error),
    ResponseParseError(reqwest::Error),
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

impl CatchApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    async fn handle_response<T: DeserializeOwned>(
        response: Response,
    ) -> Result<CatchApiResponse<T>, CatchApiError> {
        match response.status() {
            StatusCode::NO_CONTENT => Ok(CatchApiResponse::NoContent),
            _ => {
                let data = response
                    .json::<T>()
                    .await
                    .map_err(CatchApiError::ResponseParseError)?;
                Ok(CatchApiResponse::Success(data))
            }
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
