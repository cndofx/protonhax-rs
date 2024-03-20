use std::process::Command;

use anyhow::Context;

use crate::state::State;

pub fn init(command: Vec<String>) -> Result<(), anyhow::Error> {
    let state = State::init(&command)?;

    // execute command
    let args = &command[1..];
    let cmd = &command[0];
    let status = Command::new(cmd)
        .args(args)
        .status()
        .context("unable to spawn given command")?;
    let code = status.code().unwrap_or(0);

    // cleanup
    std::fs::remove_dir_all(state.dir()).context("unable to delete protonhax dir")?;
    std::process::exit(code);
}
