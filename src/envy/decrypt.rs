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
            Line::Meta(Meta::Encrypted) => {
                result.push(Line::Meta(Meta::Encrypt));

                match lines.next() {
                    Some(Line::Env(ref env)) => {
                        result.push(Line::Env(encrypter.decrypt(env).map_err(|e| {
                            anyhow::Error::msg(format!("decrypt error: {}", e.to_string()))
                        })?));
                    }
                    _ => return Err(anyhow::Error::msg("expect Env")),
                }
            }
            Line::Meta(Meta::Comment(_)) | Line::Env(_) => {
                result.push(line);
            }
            Line::Meta(Meta::Encrypt) => return Err(anyhow::Error::msg("unexpected ENCRYPT meta")),
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
