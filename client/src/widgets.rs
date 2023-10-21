use ratatui::style::{Modifier, Style};
use ratatui::widgets::Block;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::theme;

#[derive(Clone)]
pub struct InputField {
    prompt: String,
    value: String,
    cursor: usize,
    style: Style,
    block: Option<Block<'static>>,
}

impl InputField {
    pub fn new(prompt: &str) -> Self {
        InputField {
            prompt: prompt.to_string(),
            value: String::new(),
            cursor: 0usize,
            style: Style::default(),
            block: None,
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn set_cursor(&mut self, pos: usize) {
        self.cursor = pos;
    }

    pub fn get_cursor(&self) -> usize {
        self.cursor
    }

    pub fn decrement_cursor(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn increment_cursor(&mut self) {
        if self.cursor < self.value.len() {
            self.cursor += 1;
        }
    }

    pub fn width(&self) -> u16 {
        self.prompt.len() as u16 + self.value.len() as u16
    }

    pub fn height(&self) -> u16 {
        1u16
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn block(mut self, block: Block<'static>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn default_style(self) -> Self {
        self.style(theme::prompt_field_style())
            .block(theme::prompt_field_block())
    }
}

impl Widget for InputField {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(block) = self.block {
            block.render(area, buf);
        }

        let value = format!("{}: {}", self.prompt, self.value);
        buf.set_string(area.left(), area.top(), &value, self.style);

        let cursor_x = area.left() + self.prompt.len() as u16 + self.cursor as u16 + 2;
        let cursor_rect = Rect::new(cursor_x, area.top(), 1, 1);
        buf.set_style(
            cursor_rect,
            Style::default().add_modifier(Modifier::REVERSED),
        );
    }
}
