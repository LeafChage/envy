use super::base64;
use aes_gcm::{
    aead::{KeyInit, OsRng},
    Aes256Gcm,
};

pub fn gen_key() -> String {
    let key = Aes256Gcm::generate_key(OsRng);
    base64::encode(&key)
}
