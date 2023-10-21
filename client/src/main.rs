use crossterm::{
    event::{self, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Rect, Stylize, Terminal},
    style::Style,
    widgets::{List, ListItem, Paragraph},
};
use std::io::{stderr, Result};

mod handlers;
mod statics;
mod theme;
mod util;

fn main() -> Result<()> {
    stderr().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    terminal.clear()?;

    let mut selected_option = 0usize;
    let options = vec![
        "Register",
        "View Key",
        "Send Message",
        "Receive Message",
        "Quit",
    ];

    loop {
        terminal.draw(|frame| {
            let area = frame.size();

            let logo_height = crate::statics::ASCII_ART.lines().count() as u16;

            let logo_area = Rect::new(area.x, area.y, area.width, logo_height);
            let list_area = Rect::new(
                area.x,
                area.y + logo_height,
                area.width,
                area.height - logo_height,
            );

            frame.render_widget(Paragraph::new(crate::statics::ASCII_ART), logo_area);

            let items: Vec<ListItem> = options
                .iter()
                .map(|option| {
                    ListItem::new(option.to_string()).style(
                        if option == &options[selected_option] {
                            theme::list_item_selected()
                        } else {
                            theme::list_item_default()
                        },
                    )
                })
                .collect();

            let list = List::new(items).block(
                ratatui::widgets::Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .border_style(Style::default().white()),
            );
            frame.render_widget(list, list_area);
        })?;

        if let event::Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => {
                    if selected_option > 0 {
                        selected_option -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_option < options.len() - 1 {
                        selected_option += 1;
                    }
                }
                KeyCode::Enter => match options[selected_option] {
                    "Register" => {
                        handlers::handle_register(&mut terminal)?;
                    }
                    "View Key" => {
                        terminal.draw(|frame| {
                            let area = frame.size();
                            frame.render_widget(
                                Paragraph::new("View Key").style(Style::default().white()),
                                area,
                            );
                        })?;
                    }
                    "Send Message" => {
                        terminal.draw(|frame| {
                            let area = frame.size();
                            frame.render_widget(
                                Paragraph::new("Send Message").style(Style::default().white()),
                                area,
                            );
                        })?;
                    }
                    "Read Message" => {
                        terminal.draw(|frame| {
                            let area = frame.size();
                            frame.render_widget(
                                Paragraph::new("Read Message").style(Style::default().white()),
                                area,
                            );
                        })?;
                    }
                    "Quit" => {
                        break;
                    }
                    _ => {}
                },
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    stderr().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
