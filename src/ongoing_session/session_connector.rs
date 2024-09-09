use crate::api_client::cli_entity::CatchConnectCLIResponse;
use crate::api_client::request_entity::CatchConnectCLIRequest;
use crate::api_client::{CatchApiClient, CatchApiResponse};
use log::{error, info, warn};
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::cursor::position;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::Terminal;
use std::io;
use std::time::Duration;
use tokio::select;

#[derive(Default)]
struct ConnectSessionConnectUiState {
    ui_state: throbber_widgets_tui::ThrobberState,
}

impl ConnectSessionConnectUiState {
    fn on_tick(&mut self) {
        self.ui_state.calc_next();
    }
}

fn finalize_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    terminal.show_cursor()?;

    Ok(())
}

async fn perform_api_request(
    session_id: String,
    org_name: String,
    repo_name: String,
) -> io::Result<CatchConnectCLIResponse> {
    let api_client = CatchApiClient::new();

    let response = api_client
        .post::<CatchConnectCLIResponse, CatchConnectCLIRequest>(
            "/cli",
            &CatchConnectCLIRequest {
                session_id,
                repo_owner: org_name,
                repo_name,
            },
        )
        .await
        .unwrap();

    match response {
        CatchApiResponse::Success(response) => Ok(response),
        CatchApiResponse::NoContent => {
            error!("API request failed");
            Err(io::Error::new(io::ErrorKind::Other, "API request failed"))
        }
    }
}

pub async fn connect_cli_to_session(
    session_id: String,
    org_name: String,
    repo_name: String,
) -> io::Result<CatchConnectCLIResponse> {
    enable_raw_mode()?;

    let stdout = io::stdout();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(100);
    let mut state = ConnectSessionConnectUiState::default();

    let mut api_future = tokio::spawn(perform_api_request(
        session_id.clone(),
        org_name.clone(),
        repo_name.clone(),
    ));

    let mut api_result: Option<CatchConnectCLIResponse> = None;

    let message = format!(
        "Setting repoKey({}/{}) and attaching cli to onboarding session(id={})...",
        org_name, repo_name, session_id
    );

    let terminal_size = terminal.size()?;
    let (_, row) = position()?;
    let area = Rect::new(0, row, terminal_size.width, 3);

    let mut is_canceled = false;

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
                    is_canceled = true;
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
                        break;
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

    match api_result {
        Some(result) => {
            println!(" {} - Completed", message.clone());
            Ok(result)
        }
        None => {
            if !is_canceled {
                println!(" {} - Failed", message.clone());
            }

            Err(io::Error::new(io::ErrorKind::Other, "API request failed"))
        }
    }
}
