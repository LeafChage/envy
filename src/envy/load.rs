use std::path::PathBuf;
use std::process::Command;

pub fn action(path: &PathBuf, cmd: &Vec<String>) -> Result<(), anyhow::Error> {
    let envs = super::env::parser_ignore_meta(path)?
        .into_iter()
        .map(|env| env.tuple())
        .collect::<Vec<_>>();

    if cmd.len() == 0 {
        return Err(anyhow::Error::msg("cmd is empty"));
    }

    let bin = cmd.first().unwrap();
    Command::new(bin)
        .args(cmd[1..].iter())
        .envs(envs)
        .spawn()
        .expect("failed to spawn child process");

    Ok(())
}
