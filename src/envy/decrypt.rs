use super::crypt;
use super::crypt::base64;
use super::env::{Line, Meta};
use aes_gcm::{aead::Nonce, Aes256Gcm};
use std::path::PathBuf;

pub fn action(path: &PathBuf, key: &str) -> Result<(), anyhow::Error> {
    let key = base64::decode(key)?;
    let encrypter = crypt::EnvEncrypt::init(&key[..])?;
    let lines = super::env::parser(path)?;

    let mut result = vec![];
    let mut lines = lines.into_iter();
    while let Some(line) = lines.next() {
        match line {
            Line::Meta(Meta::Encrypted(nonce)) => match lines.next() {
                Some(Line::Env(ref env)) => {
                    let nonce = base64::decode(&nonce)?;
                    let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce);

                    let env = encrypter.decrypt(env, nonce).map_err(|e| {
                        anyhow::Error::msg(format!("decrypt error: {}", e.to_string()))
                    })?;
                    result.push(Line::Meta(Meta::Encrypt));
                    result.push(Line::Env(env));
                }
                _ => return Err(anyhow::Error::msg("expect Env")),
            },
            Line::Meta(Meta::Comment(_))
            | Line::Env(_)
            | Line::Meta(Meta::Encrypt)
            | Line::Meta(Meta::WhiteSpaces) => {
                result.push(line);
            }
        }
    }

    println!(
        "{}",
        result
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );

    Ok(())
}
