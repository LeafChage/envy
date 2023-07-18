use super::crypt;
use super::crypt::base64;
use super::env::{Line, Meta};
use std::path::PathBuf;

pub fn action(path: &PathBuf, key: &str) -> Result<(), anyhow::Error> {
    let key = base64::decode(key)?;
    let encrypter = crypt::EnvEncrypt::init(&key[..])?;
    let lines = super::env::parser(path)?;

    let mut result = vec![];
    let mut lines = lines.into_iter();
    while let Some(line) = lines.next() {
        match line {
            Line::Meta(Meta::Encrypt) => {
                result.push(Line::Meta(Meta::Encrypted));

                match lines.next() {
                    Some(Line::Env(ref env)) => {
                        result.push(Line::Env(encrypter.encrypt(env)?));
                    }
                    _ => return Err(anyhow::Error::msg("expect Env")),
                }
            }
            Line::Meta(Meta::Comment(_)) | Line::Env(_) => {
                result.push(line);
            }
            Line::Meta(Meta::Encrypted) => {
                return Err(anyhow::Error::msg("unexpected ENCRYPTED meta"))
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
