use super::crypt;
use super::env::{Line, Meta};
use std::path::PathBuf;

pub fn action(path: &PathBuf, key: &str) -> Result<(), anyhow::Error> {
    let encrypter = crypt::EnvEncrypt::init(key)?;
    let lines = super::env::parser(path)?;

    let mut next_is_enc = false;
    for line in lines.into_iter() {
        match line {
            Line::Meta(Meta::Secret) => {
                next_is_enc = true;
                println!("{}", line.to_string());
            }
            Line::Meta(Meta::Comment(_)) => {
                next_is_enc = false;
                println!("{}", line.to_string());
            }
            Line::Env(ref env) => {
                if next_is_enc {
                    println!("{}", encrypter.decrypt(env)?.to_string());
                } else {
                    println!("{}", line.to_string());
                }
                next_is_enc = false;
            }
        }
    }

    Ok(())
}
