use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::Context;
use rmp_serde::Deserializer;
use serde::Deserialize;

use crate::Env;

pub fn cmd(mut protonhax_dir: PathBuf, appid: u32) -> Result<(), anyhow::Error> {
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

    // read proton exe file
    protonhax_dir.pop();
    protonhax_dir.push("exe");
    let mut exe_file = std::fs::File::open(&protonhax_dir)?;
    let mut proton_exe = String::new();
    exe_file.read_to_string(&mut proton_exe)?;

    // read proton prefix file
    protonhax_dir.pop();
    protonhax_dir.push("pfx");
    let mut pfx_file = std::fs::File::open(&protonhax_dir)?;
    let mut proton_pfx = String::new();
    pfx_file.read_to_string(&mut proton_pfx)?;

    // execute command
    let cmd_path = format!("{proton_pfx}/drive_c/windows/system32/cmd.exe");
    dbg!(&proton_pfx);
    dbg!(&cmd_path);

    let mut cmd = vec!["run".to_owned()];
    cmd.push(cmd_path);
    let status = Command::new(proton_exe)
        .args(cmd)
        .envs(env.vars)
        .status()
        .context("unable to spawn command")?;
    let code = status.code().unwrap_or(0);

    std::process::exit(code);
}
