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
            Line::Meta(Meta::Encrypt) => match lines.next() {
                Some(Line::Env(ref env)) => {
                    let (env, nonce) = encrypter.encrypt(env)?;
                    let nonce = base64::encode(&nonce);
                    result.push(Line::Meta(Meta::Encrypted(String::from(nonce))));
                    result.push(Line::Env(env));
                }
                _ => return Err(anyhow::Error::msg("expect Env")),
            },
            Line::Meta(Meta::Comment(_))
            | Line::Meta(Meta::Encrypted(_))
            | Line::Env(_)
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
