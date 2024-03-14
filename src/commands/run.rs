use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

use rmp_serde::Deserializer;
use serde::Deserialize;

use crate::Env;

pub fn run(mut protonhax_dir: PathBuf, appid: u32, mut command: Vec<String>) {
    protonhax_dir.push(appid.to_string());

    // read the environment file
    protonhax_dir.push("env");
    let mut env_file = match std::fs::File::open(&protonhax_dir) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("unable to read the environment for appid '{appid}'. is the app running?");
            std::process::exit(1);
        }
    };
    let mut buf = Vec::new();
    env_file.read_to_end(&mut buf).unwrap();
    let env = Env::deserialize(&mut Deserializer::new(&*buf)).unwrap();

    // read proton exe file
    protonhax_dir.pop();
    protonhax_dir.push("exe");
    let mut exe_file = std::fs::File::open(&protonhax_dir).unwrap();
    let mut proton_exe = String::new();
    exe_file.read_to_string(&mut proton_exe).unwrap();

    // execute command
    let mut cmd = vec!["run".to_owned()];
    cmd.append(&mut command);
    let status = match Command::new(proton_exe).args(cmd).envs(env.vars).status() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("unable to spawn command: {e}");
            std::process::exit(1);
        }
    };
    let code = status.code().unwrap_or(0);

    std::process::exit(code);
}
