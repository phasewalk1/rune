use ratatui::style::{Color, Modifier, Style, Stylize};

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
