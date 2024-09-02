use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CatchConnectCLIResponse {
    pub public_key: String,
    pub integration_id: String,
}
