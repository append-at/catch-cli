use crate::api_client::request_entity::CatchCLIUploadFilesRequest;
use crate::api_client::{CatchApiClient, CatchApiResponse};
use crate::code_reader::CatchCLICodeFile;
use crate::cryptography::encrypt_rsa4096_base64_bytes;
use crate::terminal::finalize_terminal;
use log::error;
use ratatui::crossterm::cursor::position;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::{DefaultTerminal, Frame};
use std::io;
use std::time::Duration;
use tokio::select;

async fn perform_api_request(
    integration_id: String,
    session_id: String,
    code_files: Vec<CatchCLICodeFile>,
    key: [u8; 32],
    iv: [u8; 16],
    public_key_pem: String,
) -> io::Result<()> {
    let api_client = CatchApiClient::default();

    let response = api_client
        .post::<(), CatchCLIUploadFilesRequest>(
            format!("/cli/{}/files", integration_id).as_str(),
            &CatchCLIUploadFilesRequest {
                session_id,
                files: code_files,
                client_encrypted_iv: encrypt_rsa4096_base64_bytes(&public_key_pem, &iv)
                    .unwrap_or("".to_string()),
                client_encrypted_key: encrypt_rsa4096_base64_bytes(&public_key_pem, &key)
                    .unwrap_or("".to_string()),
            },
        )
        .await;

    let response = match response {
        Ok(response) => response,
        Err(e) => {
            error!("API request failed: {}", format!("{:?}", e));
            return Err(io::Error::new(io::ErrorKind::Other, "API request failed"));
        }
    };

    match response {
        CatchApiResponse::Success(_) => {
            error!("API request failed");
            Err(io::Error::new(io::ErrorKind::Other, "API request failed"))
        }
        CatchApiResponse::NoContent => Ok(()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct CodeUploader {
    state: throbber_widgets_tui::ThrobberState,
}

impl CodeUploader {
    fn on_tick(&mut self) {
        self.state.calc_next();
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn run(
        mut self,
        mut terminal: DefaultTerminal,
        integration_id: String,
        session_id: String,
        code_files: Vec<CatchCLICodeFile>,
        key: [u8; 32],
        iv: [u8; 16],
        public_key_pem: String,
    ) -> io::Result<()> {
        let tick_rate = Duration::from_millis(100);

        let terminal_size = terminal.size()?;
        let (_, row) = position()?;
        let area = Rect::new(0, row, terminal_size.width, 3);

        let mut api_future = tokio::spawn(perform_api_request(
            integration_id.clone(),
            session_id.clone(),
            code_files.clone(),
            key,
            iv,
            public_key_pem.clone(),
        ));

        loop {
            terminal.draw(|frame| self.draw(frame, area))?;

            select! {
                _ = tokio::time::sleep(tick_rate) => {
                    self.on_tick();
                }
                api_resp = &mut api_future => {
                    match api_resp {
                        Ok(_result) => {
                            finalize_terminal(&mut terminal)?;
                            break;
                        }
                        Err(e) => {
                            finalize_terminal(&mut terminal)?;
                            return Err(io::Error::new(io::ErrorKind::Other, format!("API request failed: {}", e)));
                        }
                    }
                }
            }
        }

        finalize_terminal(&mut terminal)?;

        println!(" Uploading your code with E2EE encryption... - Completed");
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let throbber = throbber_widgets_tui::Throbber::default()
            .label("Uploading your code with E2EE encryption...")
            .throbber_set(throbber_widgets_tui::BRAILLE_SIX)
            .throbber_style(ratatui::style::Style::default().bold());

        frame.render_stateful_widget(throbber, area, &mut self.state);
    }
}
