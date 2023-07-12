use super::crypt::key;

pub fn action() -> Result<(), anyhow::Error> {
    println!("{}", key::gen_key());
    Ok(())
}
