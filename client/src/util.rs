use crate::widgets::InputField;
use crossterm::event::{self, KeyCode};
use ratatui::{
    prelude::{CrosstermBackend, Rect},
    terminal::CompletedFrame,
    Terminal,
};
use std::io::Stderr;

pub type LogoArea = Rect;
pub type ListArea = Rect;

pub fn get_areas(frame_area: Rect) -> (LogoArea, ListArea) {
    use crate::statics::ASCII_ART as LOGO;

    let logo_height = LOGO.lines().count() as u16;
    let logo_width = LOGO.lines().map(|line| line.len()).max().unwrap_or(0) as u16;

    let centered_x =
        ((frame_area.width.saturating_sub(logo_width)) / 2).min(frame_area.width - logo_width);
    let logo_area = Rect::new(centered_x, frame_area.y, logo_width, logo_height);
    let list_area = Rect::new(
        frame_area.x,
        frame_area.y + logo_height,
        frame_area.width,
        frame_area.height - logo_height,
    );

    (logo_area, list_area)
}

pub fn read_input(
    term: &mut Terminal<CrosstermBackend<Stderr>>,
    mut field: InputField,
) -> Option<String> {
    let mut input = String::new();

    let placeholder = "Enter text...";

    loop {
        if let event::Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char(c) => {
                    field.increment_cursor();
                    input.insert(field.get_cursor(), c);
                }
                KeyCode::Enter => {
                    if input.is_empty() {
                        return None;
                    } else {
                        return Some(input);
                    }
                }
                KeyCode::Backspace => {
                    if field.get_cursor() > 0 {
                        input.remove(field.get_cursor() - 1);
                        field.decrement_cursor();
                    }
                }
                KeyCode::Left => field.decrement_cursor(),
                KeyCode::Right => field.increment_cursor(),
                _ => {}
            }

            term.draw(|frame| {
                let area = frame.size();

                let centered_x = ((area.width.saturating_sub(field.width())) / 2)
                    .min(area.width - field.width());
                let centered_y = ((area.height.saturating_sub(field.height())) / 2)
                    .min(area.height - field.height());
                let area = Rect::new(centered_x, centered_y, field.width(), field.height());

                let input_display = if input.is_empty() {
                    placeholder
                } else {
                    &input
                };

                field.set_value(input_display.to_owned());
                frame.render_widget(field.clone(), area);
            })
            .unwrap();
        }
    }
}
