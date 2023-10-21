use std::io::Stderr;

use ratatui::{
    prelude::CrosstermBackend,
    style::{Style, Stylize},
    widgets::Paragraph,
    Terminal,
};
use rune_core::{
    cmd::register::register_create_home,
    crypto::keyring::{Keyring, KeyringEncryptor},
};

pub fn handle_register(
    term: &mut Terminal<CrosstermBackend<Stderr>>,
) -> Result<(), std::io::Error> {
    term.draw(|frame| {
        let area = frame.size();
        frame.render_widget(
            Paragraph::new("Register").style(Style::default().white()),
            area,
        );
    })?;
    let username = super::util::read_input(term, "Enter username");
    if let Some(user) = username {
        register_create_home(&user)?;
        let keypair = Keyring::generate();
        keypair.save_public_key(&user)?;

        let passphrase = super::util::read_input(term, "Enter passphrase to encrypt keyring");

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
