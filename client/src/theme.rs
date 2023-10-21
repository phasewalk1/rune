use ratatui::{
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, BorderType, Borders},
};

const PRIMARY: Color = Color::DarkGray;
const SECONDARY: Color = Color::Cyan;
const BACKGROUND: Color = Color::Black;
const ACCENT: Color = Color::Yellow;
const TEXT: Color = Color::White;

pub fn list_item_default() -> Style {
    Style::default().fg(TEXT).bg(PRIMARY)
}

pub fn list_item_selected() -> Style {
    Style::default()
        .fg(BACKGROUND)
        .bg(SECONDARY)
        .add_modifier(Modifier::BOLD)
}

pub fn prompt_field_style() -> Style {
    Style::default().fg(Color::White).bg(Color::Blue)
}

pub fn prompt_field_block() -> Block<'static> {
    Block::default()
        .borders(Borders::LEFT | Borders::RIGHT)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Blue))
}
