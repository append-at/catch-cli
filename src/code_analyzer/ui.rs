use crate::api_client::session_status_entity::CatchSessionExtractingCandidatesResult;
use crate::api_client::CatchApiResponse;
use crate::code_analyzer::{check_rcp_status, request_rcp};
use crate::code_reader::CatchCLICodeFile;
use crate::terminal::finalize_terminal;
use log::{error, info, warn};
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::cursor::position;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::crossterm::terminal::enable_raw_mode;
use ratatui::layout::Rect;
use ratatui::prelude::Stylize;
use ratatui::Terminal;
use std::io;
use std::time::Duration;
use tokio::select;

#[derive(Default)]
struct CodeCandidateUiState {
    ui_state: throbber_widgets_tui::ThrobberState,
}

impl CodeCandidateUiState {
    fn on_tick(&mut self) {
        self.ui_state.calc_next();
    }
}

pub async fn request_code_candidates(
    integration_id: String,
    session_id: String,
    code_files: Vec<CatchCLICodeFile>,
) -> io::Result<CatchSessionExtractingCandidatesResult> {
    enable_raw_mode()?;

    let stdout = io::stdout();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(100);
    let mut state = CodeCandidateUiState::default();

    let mut api_result: Option<CatchApiResponse<()>> = None;

    let message = "Analyzing your code structure...".to_string();

    let terminal_size = terminal.size()?;
    let (_, row) = position()?;
    let area = Rect::new(0, row, terminal_size.width, 3);

    let mut api_future = tokio::spawn(request_rcp(integration_id, session_id.clone(), code_files));

    loop {
        terminal.draw(|f| {
            let throbber = throbber_widgets_tui::Throbber::default()
                .label(message.clone())
                .throbber_set(throbber_widgets_tui::BRAILLE_SIX)
                .throbber_style(ratatui::style::Style::default().bold());
            f.render_stateful_widget(throbber, area, &mut state.ui_state);
        })?;

        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    finalize_terminal(&mut terminal)?;
                    println!(" {} - Canceled", message.clone());
                    warn!("User aborted the operation");

                    break;
                }
            }
        }

        select! {
            _ = tokio::time::sleep(tick_rate) => {
                state.on_tick();
            }
            api_resp = &mut api_future => {
                match api_resp {
                    Ok(result) => {
                        api_result = Some(result?);

                        match api_result {
                            Some(ref result) => {
                                match result {
                                    CatchApiResponse::NoContent => {
                                        break;
                                    }
                                    _ => {
                                        return Err(io::Error::new(io::ErrorKind::Other, "Invalid response"));
                                    }
                                }
                            }
                            None => {
                                error!("Failed to get code candidates");
                                return Err(io::Error::new(io::ErrorKind::Other, "Failed to get code candidates"));
                            }
                        }
                    }
                    Err(e) => {
                        info!("API request unsuccessful");
                        return Err(io::Error::new(io::ErrorKind::Other, format!("API request failed: {}", e)));
                    }
                }
            }
        }
    }

    finalize_terminal(&mut terminal)?;

    if api_result.is_some() {
        match check_rcp_status(session_id.clone()).await {
            Ok(result) => {
                if result.status == "completed" {
                    Ok(result)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "API request failed"))
                }
            }
            Err(e) => {
                error!("Failed to check session status: {}", e);
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to check session status",
                ))
            }
        }
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "API request failed"))
    }
}
