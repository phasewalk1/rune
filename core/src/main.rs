use clap::App;

mod cmd;
mod crypto;

use cmd::key::KeyCmd;
use cmd::register::{prompt_passphrase, register_create_home, RegisterCmd};
use cmd::send::{ReceiveCmd, SendCmd};
use crypto::keyring::Keyring;

use crate::crypto::keyring::KeyringEncryptor;

fn main() {
    pretty_env_logger::try_init().ok();

    let matches = App::new("rune")
        .about("e2ee messenger protocol")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("@phasewalk1 <3")
        .subcommand(RegisterCmd::default().0)
        .subcommand(KeyCmd::default().0)
        .subcommand(SendCmd::default().0)
        .subcommand(ReceiveCmd::default().0)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("register") {
        let username = matches.value_of("username").unwrap();
        log::debug!("received username arg: {}", username);
        // TODO:
        // Implement some username uniqueness check with the backend server once
        // we have that up and running
        log::debug!("creating a home for user at $HOME/.config/rune");
        register_create_home(&username).unwrap();
        log::debug!("created $HOME/.config/rune/{}", username);

        // generate a keypair
        let keypair = Keyring::generate();
        log::debug!("generated keypair: {:?}", keypair);
        keypair.save_public_key(username).unwrap();
        // save secret key to a file, prompt user for passphrase to encrypt it
        let passphrase = prompt_passphrase().unwrap();

        let encryptor = KeyringEncryptor::from(keypair);
        let home_dir = dirs::home_dir().unwrap();

        let path = format!(
            "{}/.config/rune/{}/keyring.enc",
            home_dir.display(),
            username
        );
        std::fs::create_dir_all(format!("{}/.config/rune/{}", home_dir.display(), username))
            .unwrap();
        encryptor.encrypt(&path, &passphrase).unwrap();
        log::debug!("saved encrypted keyring to {}", path);
    }

    if let Some(matches) = matches.subcommand_matches("key") {
        let display = matches.value_of("display");
        if let Some(_) = display {
            // TODO: hack
            let enc_path = dirs::home_dir()
                .unwrap()
                .join(".config/rune/phasewalk1/keyring.enc");
            if let Some(enc_path) = enc_path.to_str() {
                let keyring =
                    KeyringEncryptor::decrypt(enc_path, &prompt_passphrase().unwrap()).unwrap();
                KeyCmd::unsafe_display(keyring);
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("send") {
        use crate::cmd::send::send_message;

        let username = matches.value_of("recipient").unwrap();
        let message = matches.value_of("message").unwrap();

        let keyring = Keyring::load().unwrap();

        send_message(username, message).unwrap();
    }

    if let Some(matches) = matches.subcommand_matches("receive") {
        use crate::cmd::send::receive_message;

        let sender = matches.value_of("sender").unwrap();

        let msg = receive_message(sender, "emobitstream");
        println!("{}", msg.unwrap());
    }
}
