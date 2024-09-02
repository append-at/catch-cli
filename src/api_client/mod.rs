const BASE_API_URL: &str = "https://api.dev.trycatch.ai";

use reqwest::Client;
use serde::{Deserialize, Serialize};

pub mod request_entity;
pub mod response_entity;

pub struct CatchApiClient {
    client: Client,
    base_url: String,
}

pub enum CatchApiResponse<T: for<'de> Deserialize<'de>> {
    Success(T),
    NoContent,
}

impl CatchApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
    ) -> Result<CatchApiResponse<T>, reqwest::Error> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await?;
        let converted_response = response.json::<T>().await?;

        Ok(CatchApiResponse::Success(converted_response))
    }

    pub async fn post<T: for<'de> Deserialize<'de>, U: Serialize>(
        &self,
        endpoint: &str,
        body: &U,
    ) -> Result<CatchApiResponse<T>, reqwest::Error> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.post(&url).json(body).send().await?;

        match response.status() {
            reqwest::StatusCode::NO_CONTENT => Ok(CatchApiResponse::NoContent),
            _ => {
                let data = response.json::<T>().await?;
                Ok(CatchApiResponse::Success(data))
            }
        }
    }

    pub async fn delete(&self, endpoint: &str) -> Result<CatchApiResponse<()>, reqwest::Error> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.delete(&url).send().await?;

        match response.status() {
            reqwest::StatusCode::NO_CONTENT => Ok(CatchApiResponse::NoContent),
            _ => Ok(CatchApiResponse::Success(())),
        }
    }
}
