use crossterm::event::{self, KeyCode};
use ratatui::{
    prelude::CrosstermBackend,
    style::{Style, Stylize},
    widgets::Paragraph,
    Terminal,
};
use std::io::Stderr;

pub fn draw_brand(term: &mut Terminal<CrosstermBackend<Stderr>>) -> Result<(), std::io::Error> {
    term.draw(|frame| {
        let area = frame.size();
        frame.render_widget(
            Paragraph::new(crate::statics::ASCII_ART).style(Style::default().white()),
            area,
        );
    })?;
    Ok(())
}

pub fn read_input(term: &mut Terminal<CrosstermBackend<Stderr>>, prompt: &str) -> Option<String> {
    let mut input = String::new();

    term.draw(|frame| {
        let area = frame.size();
        frame.render_widget(
            Paragraph::new(format!("{}: {}", prompt, input)).style(Style::default().white()),
            area,
        );
    })
    .unwrap();

    loop {
        if let event::Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char(c) => {
                    input.push(c);
                    term.draw(|frame| {
                        let area = frame.size();
                        frame.render_widget(
                            Paragraph::new(format!("{}: {}", prompt, input))
                                .style(Style::default().white()),
                            area,
                        );
                    })
                    .unwrap();
                }
                KeyCode::Enter => {
                    if input.is_empty() {
                        return None;
                    } else {
                        return Some(input);
                    }
                }
                KeyCode::Backspace => {
                    input.pop();
                    term.draw(|frame| {
                        let area = frame.size();
                        frame.render_widget(
                            Paragraph::new(format!("{}: {}", prompt, input))
                                .style(Style::default().white()),
                            area,
                        );
                    })
                    .unwrap();
                }
                _ => {}
            }
        }
    }
}
