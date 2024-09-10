use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::terminal::disable_raw_mode;
use ratatui::Terminal;
use std::io;

pub fn finalize_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    terminal.show_cursor()?;

    Ok(())
}
