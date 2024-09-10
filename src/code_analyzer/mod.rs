pub mod ui;

use crate::api_client::request_entity::CatchCLIRcpRequest;
use crate::api_client::session_status_entity::{
    CatchSessionExtractingCandidatesResult, CatchSessionStatusResponse,
};
use crate::api_client::{CatchApiClient, CatchApiResponse};
use crate::code_reader::CatchCLICodeFile;
use std::io;

pub async fn request_rcp(
    integration_id: String,
    session_id: String,
    code_files: Vec<CatchCLICodeFile>,
) -> io::Result<CatchApiResponse<()>> {
    let file_paths: Vec<String> = code_files
        .into_iter()
        .map(|file| file.path.clone())
        .collect();

    let api_client = CatchApiClient::default();

    let response = api_client
        .post::<(), CatchCLIRcpRequest>(
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
            _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid response")),
        },
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", e))),
    }
}

pub async fn check_rcp_status(
    session_id: String,
) -> io::Result<CatchSessionExtractingCandidatesResult> {
    let api_client = CatchApiClient::default();

    let response = api_client
        .get::<CatchSessionStatusResponse>(format!("/session/{}/process", session_id).as_str())
        .await;

    match response {
        Ok(rst) => match rst {
            CatchApiResponse::Success(response) => {
                if let Some(output) = response.process.output {
                    Ok(output.extracting_candidates)
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("{:?}", "Invalid response"),
                    ))
                }
            }
            CatchApiResponse::NoContent => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("{:?}", "Invalid response"),
            )),
        },
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", e))),
    }
}
