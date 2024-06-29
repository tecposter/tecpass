mod aes;
mod argon2;

pub use self::aes::AesCipher;
pub use self::argon2::Argon2Cipher;

#[allow(dead_code)]
const NONCE_LEN: usize = 12;
