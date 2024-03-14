use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use rmp_serde::Serializer;
use serde::Serialize;

use crate::Env;

pub fn init(mut protonhax_dir: PathBuf, command: Vec<String>) {
    // create protonhax_dir
    let appid = match std::env::var("SteamAppId") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("unable to read $SteamAppId, are you running this command through steam?");
            std::process::exit(1);
        }
    };
    protonhax_dir.push(appid);
    std::fs::create_dir_all(&protonhax_dir).expect("unable to create protonhax directory");

    // get prefix path
    let compat_dir = match std::env::var("STEAM_COMPAT_DATA_PATH") {
        Ok(v) => v,
        Err(_) => {
            eprintln!(
                "unable to read $STEAM_COMPAT_DATA_PATH, are you running this command through steam?"
            );
            std::process::exit(1);
        }
    };
    let mut compat_dir = PathBuf::from(compat_dir);
    compat_dir.push("pfx");

    // get proton path
    let proton_exe = command
        .iter()
        .find(|s| s.contains("/proton"))
        .expect("unable to find proton path in launch command");

    // write prefix path
    let mut file_path = protonhax_dir.clone();
    file_path.push("pfx");
    let mut file = std::fs::File::create(&file_path).unwrap();
    write!(file, "{}", compat_dir.display()).unwrap();

    // write proton path
    file_path.pop();
    file_path.push("exe");
    let mut file = std::fs::File::create(&file_path).unwrap();
    write!(file, "{}", proton_exe).unwrap();

    // write env
    file_path.pop();
    file_path.push("env");
    let mut file = std::fs::File::create(&file_path).unwrap();
    let vars: Vec<(String, String)> = std::env::vars().collect();
    let env = Env { vars };
    let mut buf = Vec::new();
    env.serialize(&mut Serializer::new(&mut buf)).unwrap();
    file.write_all(&buf).unwrap();

    // execute command
    let args = &command[1..];
    let cmd = &command[0];
    let status = Command::new(cmd)
        .args(args)
        .status()
        .expect("unable to spawn given command");
    let code = status.code().unwrap_or(0);

    // cleanup
    std::fs::remove_dir_all(protonhax_dir).expect("unable to delete protonhax dir");
    std::process::exit(code);
}
