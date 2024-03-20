use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Context;
use rmp_serde::Deserializer;
use serde::Deserialize;

use crate::Env;

pub fn exec(
    mut protonhax_dir: PathBuf,
    appid: u32,
    command: Vec<String>,
) -> Result<(), anyhow::Error> {
    protonhax_dir.push(appid.to_string());

    // read the environment file
    protonhax_dir.push("env");
    let mut env_file = std::fs::File::open(&protonhax_dir).with_context(|| {
        format!("unable to open the environment for appid '{appid}'. is the app running?")
    })?;
    let mut buf = Vec::new();
    env_file.read_to_end(&mut buf)?;
    let env = Env::deserialize(&mut Deserializer::new(&*buf))
        .context("unable to deserialize the environment file")?;

    let args = &command[1..];
    let cmd = &command[0];
    let status = Command::new(cmd)
        .args(args)
        .envs(env.vars)
        .status()
        .context("unable to spawn given command")?;
    let code = status.code().unwrap_or(0);

    std::process::exit(code);
}
