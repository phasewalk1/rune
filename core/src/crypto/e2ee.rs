use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use curve25519_dalek::edwards::EdwardsPoint;
use curve25519_dalek::montgomery::MontgomeryPoint;
use curve25519_dalek::scalar::Scalar;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::Digest;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn encrypt_message_for(
    recipient_public: &EdwardsPoint,
    message: &[u8],
) -> (Vec<u8>, MontgomeryPoint) {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    let ephemeral_private = Scalar::from_bytes_mod_order(bytes);
    let ephemeral_public =
        (ephemeral_private * curve25519_dalek::constants::ED25519_BASEPOINT_POINT).to_montgomery();

    let shared_secret_point = recipient_public.to_montgomery() * ephemeral_private;
    let shared_secret = sha2::Sha256::digest(shared_secret_point.as_bytes());

    let cipher = Aes256Cbc::new_from_slices(&shared_secret, &[0u8; 16]).unwrap();
    let encrypted = cipher.encrypt_vec(message);

    (encrypted, ephemeral_public)
}

pub fn decrypt_message_from(
    private_key: &Scalar,
    ciphertext: &[u8],
    ephemeral_public: MontgomeryPoint,
) -> Vec<u8> {
    let shared_secret_point = ephemeral_public * private_key;
    let shared_secret = sha2::Sha256::digest(shared_secret_point.as_bytes());

    let cipher = Aes256Cbc::new_from_slices(&shared_secret, &[0u8; 16]).unwrap();
    cipher.decrypt_vec(ciphertext).unwrap()
}
