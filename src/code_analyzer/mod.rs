use crate::api_client::request_entity::CatchCLIRcpRequest;
use crate::api_client::{CatchApiClient, CatchApiError, CatchApiResponse};
use crate::code_reader::CatchCLICodeFile;

pub async fn request_rcp(
    integration_id: &str,
    session_id: &str,
    code_files: &Vec<CatchCLICodeFile>,
) -> Result<CatchApiResponse<()>, CatchApiError> {
    let file_paths: Vec<String> = code_files.into_iter().map(|file| file.path).collect();

    let api_client = CatchApiClient::new();

    let response = api_client
        .post::<_, CatchCLIRcpRequest>(
            format!("/cli/{}/rcp", integration_id).as_str(),
            &CatchCLIRcpRequest {
                session_id: session_id.to_string(),
                files: file_paths,
            },
        )
        .await;

    match response {
        Ok(rst) => match rst {
            CatchApiResponse::NoContent => Ok(CatchApiResponse::NoContent),
        },
        Err(e) => Err(e),
    }
}
