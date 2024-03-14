use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Context;
use rmp_serde::Serializer;
use serde::Serialize;

use crate::Env;

pub fn init(mut protonhax_dir: PathBuf, command: Vec<String>) -> Result<(), anyhow::Error> {
    // create protonhax_dir
    let appid = std::env::var("SteamAppId")
        .context("unable to read $SteamAppId, are you running this command through steam?")?;
    protonhax_dir.push(appid);
    std::fs::create_dir_all(&protonhax_dir).context("unable to create protonhax directory")?;

    // get prefix path
    let compat_dir = std::env::var("STEAM_COMPAT_DATA_PATH").context(
        "unable to read $STEAM_COMPAT_DATA_PATH, are you running this command through steam?",
    )?;
    let mut compat_dir = PathBuf::from(compat_dir);
    compat_dir.push("pfx");

    // get proton path
    let proton_exe = command
        .iter()
        .find(|s| s.contains("/proton"))
        .context("unable to find proton path in launch command")?;

    // write prefix path
    let mut file_path = protonhax_dir.clone();
    file_path.push("pfx");
    let mut file = std::fs::File::create(&file_path).context("unable to create pfx file")?;
    write!(file, "{}", compat_dir.display())?;

    // write proton path
    file_path.pop();
    file_path.push("exe");
    let mut file = std::fs::File::create(&file_path).context("unable to create exe file")?;
    write!(file, "{}", proton_exe)?;

    // write env
    file_path.pop();
    file_path.push("env");
    let mut file = std::fs::File::create(&file_path).context("unable to create env file")?;
    let vars: Vec<(String, String)> = std::env::vars().collect();
    let env = Env { vars };
    let mut buf = Vec::new();
    env.serialize(&mut Serializer::new(&mut buf))?;
    file.write_all(&buf)?;

    // execute command
    let args = &command[1..];
    let cmd = &command[0];
    let status = Command::new(cmd)
        .args(args)
        .status()
        .context("unable to spawn given command")?;
    let code = status.code().unwrap_or(0);

    // cleanup
    std::fs::remove_dir_all(protonhax_dir).context("unable to delete protonhax dir")?;
    std::process::exit(code);
}
