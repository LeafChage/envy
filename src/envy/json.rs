use std::collections::HashMap;
use std::path::PathBuf;

pub fn action(path: &PathBuf) -> Result<(), anyhow::Error> {
    let envs = super::env::parser_ignore_meta(path)?;
    let mut hashmap = HashMap::new();
    for env in envs.iter() {
        hashmap.insert(env.key().clone(), env.value().clone());
    }

    let output = serde_json::to_string(&hashmap)?;
    println!("{}", output);
    Ok(())
}
