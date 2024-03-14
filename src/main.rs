use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};

use args::Args;

use crate::args::Command;
use crate::commands::*;

mod args;
mod commands;

#[derive(Debug, Deserialize, Serialize)]
pub struct Env {
    pub vars: Vec<(String, String)>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let runtime_dir = match std::env::var("XDG_RUNTIME_DIR") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("unable to read $XDG_RUNTIME_DIR: {e}");
            std::process::exit(1);
        }
    };
    let mut protonhax_dir = PathBuf::from(runtime_dir);
    protonhax_dir.push("protonhax-rs");

    match args.command {
        Command::Init { command } => init(protonhax_dir, command)?,
        Command::Run { appid, command } => run(protonhax_dir, appid, command)?,
        Command::Ls => ls(protonhax_dir)?,
        _ => todo!(),
    }

    Ok(())
}
