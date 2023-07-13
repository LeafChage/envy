use super::crypt;
use super::env::{Line, Meta};
use std::path::PathBuf;

pub fn action(path: &PathBuf, key: &str) -> Result<(), anyhow::Error> {
    let encrypter = crypt::EnvEncrypt::init(key)?;
    let lines = super::env::parser(path)?;

    let mut next_is_enc = false;
    for line in lines.into_iter() {
        match line {
            Line::Meta(Meta::Encrypt) => {
                next_is_enc = true;
                println!("{}", Meta::Encrypted.to_string());
            }
            Line::Meta(Meta::Encrypted) => {
                return Err(anyhow::Error::msg("unexpected ENCRYPTED meta"))
            }
            Line::Meta(Meta::Comment(_)) => {
                next_is_enc = false;
                println!("{}", line.to_string());
            }
            Line::Env(ref env) => {
                if next_is_enc {
                    println!("{}", encrypter.encrypt(env)?.to_string());
                } else {
                    println!("{}", line.to_string());
                }
                next_is_enc = false;
            }
        }
    }

    Ok(())
}
