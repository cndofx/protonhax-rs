use std::process::Command;

use anyhow::Context;

use crate::state::State;

pub fn exec(appid: u32, command: Vec<String>) -> Result<(), anyhow::Error> {
    let state = State::load(appid)?;

    let args = &command[1..];
    let cmd = &command[0];
    let status = Command::new(cmd)
        .args(args)
        .envs(state.env()?.vars)
        .status()
        .context("unable to spawn given command")?;
    let code = status.code().unwrap_or(0);

    std::process::exit(code);
}
