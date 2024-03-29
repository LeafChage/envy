use std::path::PathBuf;
use std::process::Command;

pub fn action(paths: &Vec<PathBuf>, cmd: &Vec<String>) -> Result<(), anyhow::Error> {
    let mut envs = vec![];
    for path in paths.into_iter() {
        envs.append(
            &mut super::env::parser_ignore_meta(path)?
                .into_iter()
                .map(|env| env.tuple())
                .collect::<Vec<_>>(),
        );
    }

    if cmd.len() == 0 {
        return Err(anyhow::Error::msg("cmd is empty"));
    }

    let bin = cmd.first().unwrap();
    let mut p = Command::new(bin).args(cmd[1..].iter()).envs(envs).spawn()?;
    p.wait()?;

    Ok(())
}
