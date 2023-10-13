use std::io::Write;

use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use curve25519_dalek::{
    edwards::{CompressedEdwardsY, EdwardsPoint},
    Scalar,
};
use hmac::Hmac;
use pbkdf2::pbkdf2;
use rand::{rngs::OsRng, RngCore};
use sha2::Sha256;

#[derive(Debug)]
pub struct Keyring {
    pub public: EdwardsPoint,
    pub private: Scalar,
}

impl Keyring {
    pub fn generate() -> Keyring {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        let private = Scalar::from_bytes_mod_order(bytes);
        let public = private * &curve25519_dalek::constants::ED25519_BASEPOINT_POINT;
        Keyring { public, private }
    }

    pub fn construct(pk: EdwardsPoint, sk: Scalar) -> Keyring {
        Self {
            public: pk,
            private: sk,
        }
    }

    pub fn save_public_key(&self, username: &str) -> std::io::Result<()> {
        let public_key_bytes = self.public.compress().to_bytes();
        let path = dirs::home_dir()
            .unwrap()
            .join(format!(".config/rune/{}/public-key.pub", username));

        let mut file = std::fs::File::create(path.clone())?;
        log::debug!("writing public key to {}", path.display());
        file.write_all(&public_key_bytes)
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        use crate::cmd::register::prompt_passphrase;

        let me_path = dirs::home_dir().unwrap().join(".config/rune/me");
        // read me_path
        let username = std::fs::read_to_string(me_path)?.trim().to_string();

        let path = dirs::home_dir()
            .unwrap()
            .join(format!(".config/rune/{}/keyring.enc", username));
        let decryptor =
            KeyringEncryptor::decrypt(path.to_str().unwrap(), &prompt_passphrase().unwrap());
        decryptor
    }

    pub fn load_custom(username: &str) -> Result<Self, Box<dyn std::error::Error>> {
        use crate::cmd::register::prompt_passphrase;

        let path = dirs::home_dir()
            .unwrap()
            .join(format!(".config/rune/{}/keyring.enc", username));
        KeyringEncryptor::decrypt(path.to_str().unwrap(), &prompt_passphrase().unwrap())
    }
}

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

// Can encrypt a keyring using a passphrase so users can store their keys
// encrypted locally and decrypt with a passphrase
pub struct KeyringEncryptor {
    keyring: Keyring,
}

impl From<Keyring> for KeyringEncryptor {
    fn from(keyring: Keyring) -> Self {
        Self { keyring }
    }
}

impl KeyringEncryptor {
    pub fn encrypt(&self, out_path: &str, pass: &str) -> Result<(), std::io::Error> {
        let mut salt = [0u8; 16];
        let mut iv = [0u8; 16];
        OsRng.fill_bytes(&mut salt);
        OsRng.fill_bytes(&mut iv);

        // derive a key from passphrase
        let mut derived_key = [0u8; 32];
        let _ = pbkdf2::<Hmac<Sha256>>(pass.as_bytes(), &salt, 10_000, &mut derived_key).unwrap();

        let public_bytes = self.keyring.public.compress().to_bytes();

        let private_bytes = self.keyring.private.to_bytes();

        let mut combined = Vec::new();
        combined.extend_from_slice(&public_bytes);
        combined.extend_from_slice(&private_bytes);

        let cipher = Aes256Cbc::new_from_slices(&derived_key, &iv).unwrap();
        let encrypted = cipher.encrypt_vec(&combined);

        // move these into their own struct and serde::Serialize
        let mut output = Vec::new();
        output.extend_from_slice(&salt);
        output.extend_from_slice(&iv);
        output.extend_from_slice(&encrypted);

        log::debug!("writing encrypted keyring to {}", out_path);
        let mut file = std::fs::File::create(out_path)?;
        file.write_all(output.as_ref())
    }

    pub fn decrypt(path: &str, pass: &str) -> Result<Keyring, Box<dyn std::error::Error>> {
        let data = std::fs::read(path)?;

        let salt = &data[0..16];
        let iv = &data[16..32];
        let encrypted_data = &data[32..];

        let mut key = [0u8; 32];
        pbkdf2::<Hmac<Sha256>>(pass.as_bytes(), salt, 10_000, &mut key)?;

        let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
        let decrypted = cipher.decrypt_vec(encrypted_data).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Decryption failed")
        })?;

        let public_bytes = &decrypted[0..32];
        let private_bytes = &decrypted[32..64];

        let private_bytes_arr: [u8; 32] = private_bytes.try_into().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid private key")
        })?;

        let private = Scalar::from_bytes_mod_order(private_bytes_arr);
        let public_edwards_compressed = CompressedEdwardsY::from_slice(public_bytes).unwrap();
        let public_edwards = public_edwards_compressed.decompress().unwrap();

        Ok(Keyring::construct(public_edwards, private))
    }
}
