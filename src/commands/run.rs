use std::process::Command;

use anyhow::Context;

use crate::state::State;

pub fn run(appid: u32, mut command: Vec<String>) -> Result<(), anyhow::Error> {
    let state = State::load(appid)?;

    let mut cmd = vec!["run".to_owned()];
    cmd.append(&mut command);
    let status = Command::new(state.proton()?)
        .args(cmd)
        .envs(state.env()?.vars)
        .status()
        .context("unable to spawn command")?;
    let code = status.code().unwrap_or(0);

    std::process::exit(code);
}
