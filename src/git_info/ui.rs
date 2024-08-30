use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::crossterm::{event, execute};
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Padding};
use ratatui::Terminal;
use std::io;
use std::io::StdoutLock;
use tui_textarea::{Input, Key, TextArea};
use unicode_segmentation::UnicodeSegmentation;

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

fn validate(textarea: &mut TextArea, placeholder: String) -> bool {
    let text = &textarea.lines()[0];
    let is_valid = text.graphemes(true).count() >= 2;

    if !is_valid {
        textarea.set_style(Style::default().fg(Color::LightYellow));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::LightYellow)
                .padding(Padding::new(2, 2, 1, 1))
                .title(format!(
                    " {} | ERROR: {} ",
                    placeholder, "must be length >= 2"
                )),
        );
        false
    } else {
        textarea.set_style(Style::default().fg(Color::Reset));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::Reset)
                .padding(Padding::new(2, 2, 1, 1))
                .title(format!(" {} ", placeholder)),
        );
        true
    }
}

fn open_form(
    terminal: &mut Terminal<CrosstermBackend<StdoutLock>>,
    placeholder: &str,
    default_value: &str,
) -> io::Result<String> {
    let mut textarea = TextArea::default();
    textarea.set_cursor_line_style(Style::default());
    textarea.set_placeholder_text(placeholder);
    textarea.insert_str(default_value);
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .padding(Padding::new(2, 2, 1, 1))
            .title(format!(" Enter {} ", placeholder.to_lowercase())),
    );
    let mut is_valid = validate(&mut textarea, placeholder.to_string());

    loop {
        terminal.draw(|f| {
            let area = popup_area(f.area(), 60, 60);
            f.render_widget(&textarea, area);
        })?;

        match event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            Input {
                key: Key::Enter, ..
            } if is_valid => break,
            Input {
                key: Key::Char('c'),
                ctrl: true,
                ..
            } => Err(io::Error::from(io::ErrorKind::Interrupted))?,
            Input {
                key: Key::Enter, ..
            } => {}
            input => {
                if textarea.input(input) {
                    is_valid = validate(&mut textarea, placeholder.to_string());
                }
            }
        }
    }

    let result = &textarea.lines()[0];

    Ok(result.into())
}

pub fn prompt_git_info_form(
    default_value_org_name: &str,
    default_value_repo_name: &str,
) -> io::Result<(String, String)> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture);

    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    // org_name
    let org_name = open_form(
        &mut term,
        "Enter organization name... (ex. catch-org)",
        default_value_org_name,
    )?;
    let repo_name = open_form(
        &mut term,
        "Enter repository name... (ex. catch-cli)",
        default_value_repo_name,
    )?;

    disable_raw_mode()?;
    execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    Ok((org_name, repo_name))
}
