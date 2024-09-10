use crate::code_reader::CatchCLICodeFile;
use crate::code_uploader::ui::CodeUploader;
use std::io;

mod ui;

pub async fn upload_codes(
    integration_id: String,
    session_id: String,
    code_files: Vec<CatchCLICodeFile>,
    key: [u8; 32],
    iv: [u8; 16],
    public_key_pem: String,
) -> io::Result<()> {
    let terminal = ratatui::init();

    let code_files_clone = code_files.clone();
    let upload_result = CodeUploader::default()
        .run(
            terminal,
            integration_id,
            session_id,
            code_files_clone,
            key,
            iv,
            public_key_pem,
        )
        .await;

    match upload_result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
