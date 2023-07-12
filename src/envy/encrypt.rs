use std::path::PathBuf;

pub fn action(path: &PathBuf, key: &str) -> Result<(), anyhow::Error> {
    println!("{:?}, {:?}", path, key);
    Ok(())
}
