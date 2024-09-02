use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CatchConnectCLIRequest {
    pub repo_name: String,
    pub repo_owner: String,
    pub session_id: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct CatchCLIRcpRequest {
    pub files: Vec<String>,
    #[serde(rename = "sessionId")]
    pub session_id: String,
}
