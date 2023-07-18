use super::base64;
use crate::envy::env::Env;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, Nonce, OsRng},
    Aes256Gcm, Key,
};

pub struct EnvEncrypt {
    nonce: Nonce<Aes256Gcm>,
    cipher: Aes256Gcm,
}

impl EnvEncrypt {
    pub fn init(key: &[u8]) -> Result<Self, anyhow::Error> {
        let key: [u8; 32] = key.try_into()?;
        let key = Key::<Aes256Gcm>::from_slice(&key);

        let cipher = Aes256Gcm::new(&key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        Ok(EnvEncrypt { cipher, nonce })
    }

    pub fn encrypt(&self, env: &Env) -> Result<Env, anyhow::Error> {
        let env_key = env.key();
        let data = env.value().as_bytes().as_ref();
        let raw = self
            .cipher
            .encrypt(&self.nonce, data)
            .map_err(|e| anyhow::Error::msg(e))?;
        let env_value = base64::encode(&raw);
        Ok(Env::new(env_key, env_value))
    }

    pub fn decrypt(&self, env: &Env) -> Result<Env, anyhow::Error> {
        let env_key = env.key();
        let data = base64::decode(env.value())?;
        let env_value = self
            .cipher
            .decrypt(&self.nonce, data[..].as_ref())
            .map_err(|e| anyhow::Error::msg(e))?;
        let env_value = std::str::from_utf8(env_value[..].as_ref())?;
        Ok(Env::new(env_key, env_value))
    }
}

#[cfg(test)]
mod tests {
    use super::super::base64;
    use super::super::key::gen_key;
    use super::*;
    use crate::envy::env::Env;

    #[test]
    fn it_enc() {
        let key = gen_key();
        let key = base64::decode(&key).unwrap();
        let e = EnvEncrypt::init(&key[..]).unwrap();
        let env = e.encrypt(&Env::new("key", "value")).unwrap();
        let env = e.decrypt(&env).unwrap();
        assert_eq!(env, Env::new("key", "value"));
    }
}
