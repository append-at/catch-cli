use crate::code_reader::CatchCLICodeFile;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CatchConnectCLIRequest {
    pub repo_name: String,
    pub repo_owner: String,
    pub session_id: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CatchCLIRcpRequest {
    pub files: Vec<String>,
    #[serde(rename = "sessionId")]
    pub session_id: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct CatchCLIUploadFilesRequest {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub files: Vec<CatchCLICodeFile>,
    #[serde(rename = "clientEncryptedKey")]
    pub client_encrypted_key: String,
    #[serde(rename = "clientEncryptedIv")]
    pub client_encrypted_iv: String,
}
