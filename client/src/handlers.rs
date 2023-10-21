use std::io::Stderr;

use ratatui::{
    prelude::{CrosstermBackend, Rect},
    style::{Style, Stylize},
    widgets::Paragraph,
    Terminal,
};
use rune_core::{
    cmd::register::register_create_home,
    crypto::keyring::{Keyring, KeyringEncryptor},
};

use crate::theme;

pub fn handle_register(
    term: &mut Terminal<CrosstermBackend<Stderr>>,
) -> Result<(), std::io::Error> {
    let username_input_field = crate::widgets::InputField::new("Enter username")
        .style(theme::prompt_field_style())
        .block(theme::prompt_field_block());

    term.draw(|frame| {
        let area = frame.size();

        let centered_x = ((area.width.saturating_sub(username_input_field.width())) / 2)
            .min(area.width - username_input_field.width());
        let centered_y = ((area.height.saturating_sub(username_input_field.height())) / 2)
            .min(area.height - username_input_field.height());
        let area = Rect::new(
            centered_x,
            centered_y,
            username_input_field.width(),
            username_input_field.height(),
        );

        frame.render_widget(username_input_field.clone(), area);
    })?;

    let username = super::util::read_input(term, username_input_field);

    if let Some(user) = username {
        register_create_home(&user)?;
        let keypair = Keyring::generate();
        keypair.save_public_key(&user)?;

        let passphrase_field =
            crate::widgets::InputField::new("Enter passphrase to encrypt keyring").default_style();
        let passphrase = super::util::read_input(term, passphrase_field);

        if let Some(pass) = passphrase {
            let encryptor = KeyringEncryptor::from(keypair);
            let home_dir = dirs::home_dir().unwrap();
            let path_to_keyring =
                format!("{}/.config/rune/{}/keyring.enc", home_dir.display(), user);
            std::fs::create_dir_all(format!("{}/.config/rune/{}", home_dir.display(), user))?;
            encryptor.encrypt(&path_to_keyring, &pass)?;
            return Ok(());
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No passphrase entered",
            ));
        }
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No username entered",
        ));
    }
}

pub fn handle_view_key(
    term: &mut Terminal<CrosstermBackend<Stderr>>,
) -> Result<(), std::io::Error> {
    term.draw(|frame| {
        let area = frame.size();
        frame.render_widget(
            Paragraph::new("View Key").style(Style::default().white()),
            area,
        );
    })?;

    todo!();
}
