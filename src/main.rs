use clap::Parser;
use serde::{Deserialize, Serialize};

use args::Args;

use crate::args::Command;
use crate::commands::*;

mod args;
mod commands;
mod state;

#[derive(Debug, Deserialize, Serialize)]
pub struct Env {
    pub vars: Vec<(String, String)>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    match args.command {
        Command::Init { command } => init(command)?,
        Command::Run { appid, command } => run(appid, command)?,
        Command::Exec { appid, command } => exec(appid, command)?,
        Command::Cmd { appid } => cmd(appid)?,
        Command::Ls => ls()?,
    }

    Ok(())
}
