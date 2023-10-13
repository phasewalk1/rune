use clap::{App, Arg, SubCommand};

pub struct RegisterCmd(pub(crate) App<'static>);

impl Default for RegisterCmd {
    fn default() -> Self {
        Self(
            SubCommand::with_name("register")
                .about("register a new keypair")
                .arg(
                    Arg::with_name("username")
                        .help("identity to assign to this keypair")
                        .required(true)
                        .index(1),
                ),
        )
    }
}

// Create a directory at $HOME/.config/rune/ to store user's contact list of public keys
pub(crate) fn register_create_home(username: &str) -> std::io::Result<()> {
    let home = dirs::home_dir().unwrap();
    if home.join(".config/rune").exists() {
        if home.join(format!(".config/rune/{}", username)).exists() {
            println!("Username already exists");
            std::process::exit(1);
        }
        std::fs::create_dir(home.join(format!(".config/rune/{}", username)))?;
    } else {
        std::fs::create_dir(home.join(".config/rune"))?;
        std::fs::create_dir(home.join(format!(".config/rune/{}", username)))?;
    }
    Ok(())
}

pub(crate) fn prompt_passphrase() -> std::io::Result<String> {
    use std::io::{stdin, stdout, Write};

    print!("Enter passphrase to encrypt your keyring: ");
    stdout().flush()?;

    let mut passphrase = String::new();
    stdin().read_line(&mut passphrase)?;
    Ok(passphrase.trim().to_string())
}
