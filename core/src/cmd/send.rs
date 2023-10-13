use clap::{App, Arg, SubCommand};
use curve25519_dalek::edwards::CompressedEdwardsY;
use std::fs;

pub struct SendCmd(pub(crate) App<'static>);

impl Default for SendCmd {
    fn default() -> Self {
        Self(
            SubCommand::with_name("send")
                .about("Send an encrypted message to someone")
                .arg(
                    Arg::with_name("recipient")
                        .help("Recipient's username")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("message")
                        .help("Message to send")
                        .required(true)
                        .index(2),
                ),
        )
    }
}

pub(crate) fn send_message(recipient_username: &str, message: &str) -> std::io::Result<()> {
    use crate::crypto::e2ee::encrypt_message_for;

    let home = dirs::home_dir().unwrap();
    let recipient_path = home.join(format!(".config/rune/{}", recipient_username));
    if !recipient_path.exists() {
        println!("Recipient does not exist");
        return Ok(());
    } else {
        let recipient_public_key = fs::read(recipient_path.join("public-key.pub"))?;
        let recipient_public_edwards = CompressedEdwardsY::from_slice(&recipient_public_key)
            .unwrap()
            .decompress()
            .unwrap();

        let (encrypted, ephemeral_public) =
            encrypt_message_for(&recipient_public_edwards, message.as_bytes());

        fs::write(recipient_path.join("encrypted_message.txt"), &encrypted)?;
        fs::write(
            recipient_path.join("ephemeral_public.txt"),
            ephemeral_public.as_bytes(),
        )?;

        Ok(())
    }
}

pub struct ReceiveCmd(pub(crate) App<'static>);

impl Default for ReceiveCmd {
    fn default() -> Self {
        Self(
            SubCommand::with_name("receive")
                .about("Decrypt a message sent to you")
                .arg(
                    Arg::with_name("sender")
                        .help("Sender's username")
                        .required(true)
                        .index(1),
                )
                .arg(Arg::with_name("receive_as").required(false).index(2)),
        )
    }
}

pub(crate) fn receive_message(
    sender: &str,
    receiver: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    use crate::crypto::e2ee::decrypt_message_from;
    use crate::crypto::keyring::KeyringEncryptor;
    use curve25519_dalek::montgomery::MontgomeryPoint;

    let home = dirs::home_dir().unwrap();
    let sender_path = home.join(format!(".config/rune/{}", sender));
    if !sender_path.exists() {
        println!("Sender does not exist");
        std::process::exit(1);
    }

    log::debug!("sender_path: {}", sender_path.display());

    let receiver_path = home.join(format!(".config/rune/{}", receiver));
    log::debug!("receiver_path: {}", receiver_path.display());
    let encrypted = fs::read(receiver_path.join("encrypted_message.txt"))?;
    let ephemeral_public_path = receiver_path.join("ephemeral_public.txt");
    let ephemeral_public_bytes: [u8; 32] = fs::read(ephemeral_public_path)?.try_into().unwrap();
    let ephemeral_public = MontgomeryPoint(ephemeral_public_bytes);

    // Decrypt the message
    let passphrase = crate::cmd::register::prompt_passphrase()?;
    let keyring = KeyringEncryptor::decrypt(
        &format!("{}/.config/rune/{}/keyring.enc", home.display(), receiver),
        &passphrase,
    )
    .unwrap();
    let decrypted = decrypt_message_from(&keyring.private, &encrypted, ephemeral_public);

    let msg = String::from_utf8_lossy(&decrypted);
    println!("Decrypted Message: {}", String::from_utf8_lossy(&decrypted));
    Ok(msg.to_string())
}
