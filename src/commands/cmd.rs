use std::process::Command;

use anyhow::Context;

use crate::state::State;

pub fn cmd(appid: u32) -> Result<(), anyhow::Error> {
    let state = State::load(appid)?;

    let cmd_path = format!("{}/drive_c/windows/system32/cmd.exe", state.prefix()?);

    let mut cmd = vec!["run".to_owned()];
    cmd.push(cmd_path);
    let status = Command::new(state.proton()?)
        .args(cmd)
        .envs(state.env()?.vars)
        .status()
        .context("unable to spawn command")?;
    let code = status.code().unwrap_or(0);

    std::process::exit(code);
}
