// manage keyring
//

use crate::crypto::keyring::{Keyring, KeyringEncryptor};
use clap::{App, Arg, SubCommand};

pub struct KeyCmd(pub(crate) App<'static>);

impl Default for KeyCmd {
    fn default() -> Self {
        Self(
            SubCommand::with_name("key").about("manage keyring").arg(
                Arg::with_name("display")
                    .help("export a keyring with armor")
                    .required(false)
                    .index(1),
            ),
        )
    }
}

impl KeyCmd {
    pub fn unsafe_display(keyring: Keyring) {
        println!("(unsafe!) keyring: {:?}", keyring);
    }
}
