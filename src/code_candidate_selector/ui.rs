use crate::code_reader::CatchCLICodeFile;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{
        Block, BorderType, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, TableState,
    },
    DefaultTerminal, Frame,
};
use std::io;
use std::path::Path;
use style::palette::tailwind;
use unicode_width::UnicodeWidthStr;

const INFO_TEXT: &str = "(Enter) Submit | (↑) move up | (↓) move down | (Space) Select / Unselect";

const ITEM_HEIGHT: usize = 2;

#[derive(Debug)]
struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::NEUTRAL.c950,
            header_bg: color.c900,
            header_fg: tailwind::NEUTRAL.c200,
            row_fg: tailwind::NEUTRAL.c200,
            selected_style_fg: color.c400,
            normal_row_color: tailwind::NEUTRAL.c950,
            alt_row_color: tailwind::NEUTRAL.c900,
            footer_border_color: color.c400,
        }
    }
}

struct Data {
    is_selected: String,
    file_name: String,
    file_path: String,
}

impl Data {
    const fn ref_array(&self) -> [&String; 3] {
        [&self.is_selected, &self.file_name, &self.file_path]
    }

    fn name(&self) -> &str {
        &self.file_name
    }

    fn path(&self) -> &str {
        &self.file_path
    }
}

pub struct CodeCandidateSelector {
    state: TableState,
    items: Vec<Data>,
    longest_item_lens: (u16, u16),
    scroll_state: ScrollbarState,
    colors: TableColors,
    color_index: usize,
}

impl CodeCandidateSelector {
    pub fn new(candidate_files: Vec<CatchCLICodeFile>) -> Self {
        let data_vec: Vec<Data> = candidate_files
            .into_iter()
            .map(|file| Data {
                is_selected: "0".to_string(),
                file_name: extract_filename(file.path.as_str()).to_string(),
                file_path: file.path.to_string(),
            })
            .collect();

        Self {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&data_vec),
            scroll_state: ScrollbarState::new((data_vec.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&tailwind::NEUTRAL),
            color_index: 0,
            items: data_vec,
        }
    }

    fn select(&mut self) {
        let i = self.state.selected().unwrap_or(0);
        let is_selected = match self.items[i].is_selected.as_str() {
            "0" => "1",
            _ => "0",
        };
        self.items[i].is_selected = is_selected.to_string();
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<Vec<String>> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Enter => {
                            return Ok(self
                                .items
                                .iter()
                                .filter(|data| data.is_selected == "1")
                                .map(|data| data.file_path.clone())
                                .collect())
                        }
                        KeyCode::Down => self.next(),
                        KeyCode::Up => self.previous(),
                        KeyCode::Char(' ') => self.select(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(3)]);
        let rects = vertical.split(frame.area());

        self.render_table(frame, rects[0]);
        self.render_scrollbar(frame, rects[0]);
        self.render_footer(frame, rects[1]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);
        let selected_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_style_fg);

        let header = ["Selected", "File Name", "File Path"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);
        let rows = self.items.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => self.colors.normal_row_color,
                _ => self.colors.alt_row_color,
            };
            let item = data.ref_array();
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .style(Style::new().fg(self.colors.row_fg).bg(color))
                .height(4)
        });
        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(self.longest_item_lens.0 + 1),
                Constraint::Min(self.longest_item_lens.1 + 1),
            ],
        )
        .header(header)
        .highlight_style(selected_style)
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            bar.into(),
            "".into(),
        ]))
        .bg(self.colors.buffer_bg)
        .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(t, area, &mut self.state);
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        );
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let info_footer = Paragraph::new(Line::from(INFO_TEXT))
            .style(
                Style::new()
                    .fg(self.colors.row_fg)
                    .bg(self.colors.buffer_bg),
            )
            .centered()
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .border_style(Style::new().fg(self.colors.footer_border_color)),
            );
        frame.render_widget(info_footer, area);
    }
}

fn constraint_len_calculator(items: &[Data]) -> (u16, u16) {
    let name_len = items
        .iter()
        .map(Data::name)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let path_len = items
        .iter()
        .map(Data::path)
        .flat_map(str::lines)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    #[allow(clippy::cast_possible_truncation)]
    (name_len as u16, path_len as u16)
}

fn extract_filename(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("")
}
