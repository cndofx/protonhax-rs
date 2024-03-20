use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};

use crate::Env;

pub struct State {
    path: PathBuf,
}

impl State {
    pub fn load(appid: u32) -> Result<Self, anyhow::Error> {
        let mut path = Self::base_dir()?;
        path.push(appid.to_string());
        if !path.exists() {
            Err(anyhow!(
                "unable to open state path for appid {appid}, is the app running?"
            ))
        } else {
            Ok(State { path })
        }
    }

    /// creates and populates a folder in `/run/user/<userid>/protonhax-rs/<appid>` to hold state
    ///
    /// should be called under steam via the init command
    pub fn init(steam_command: &[String]) -> Result<Self, anyhow::Error> {
        // create protonhax-rs directory
        let mut path = Self::base_dir()?;
        let appid = std::env::var("SteamAppId")
            .context("unable to read $SteamAppId, are you running this command through steam?")?;
        path.push(appid);
        std::fs::create_dir_all(&path).context("unable to create protonhax directory")?;

        // get prefix path
        let compat_dir = std::env::var("STEAM_COMPAT_DATA_PATH").context(
            "unable to read $STEAM_COMPAT_DATA_PATH, are you running this command through steam?",
        )?;
        let mut compat_dir = PathBuf::from(compat_dir);
        compat_dir.push("pfx");

        // get proton path
        let proton_exe = steam_command
            .iter()
            .find(|s| s.contains("/proton"))
            .context("unable to find proton path in launch command")?;

        // write prefix path
        path.push("pfx");
        let mut file = std::fs::File::create(&path).context("unable to create pfx file")?;
        write!(file, "{}", compat_dir.display())?;

        // write proton path
        path.pop();
        path.push("exe");
        let mut file = std::fs::File::create(&path).context("unable to create exe file")?;
        write!(file, "{}", proton_exe)?;

        // write env
        path.pop();
        path.push("env");
        let mut file = std::fs::File::create(&path).context("unable to create env file")?;
        let vars: Vec<(String, String)> = std::env::vars().collect();
        let env = Env { vars };
        let mut buf = Vec::new();
        env.serialize(&mut Serializer::new(&mut buf))?;
        file.write_all(&buf)?;

        path.pop();
        Ok(State { path })
    }

    /// returns the state's path
    pub fn dir(&self) -> &Path {
        &self.path
    }

    /// returns the contents of the `pfx` file in the state's path
    pub fn prefix(&self) -> Result<String, anyhow::Error> {
        let mut path = self.path.clone();
        path.push("pfx");

        let mut pfx_file = std::fs::File::open(&path)?;
        let mut proton_pfx = String::new();
        pfx_file.read_to_string(&mut proton_pfx)?;
        Ok(proton_pfx)
    }

    /// returns the contents of the `exe` file in the state's path
    pub fn proton(&self) -> Result<String, anyhow::Error> {
        let mut path = self.path.clone();
        path.push("exe");

        let mut exe_file = std::fs::File::open(&path)?;
        let mut proton_exe = String::new();
        exe_file.read_to_string(&mut proton_exe)?;
        Ok(proton_exe)
    }

    /// returns the contents of the `env` file in the state's path
    pub fn env(&self) -> Result<Env, anyhow::Error> {
        let mut path = self.path.clone();
        path.push("env");

        let mut env_file = std::fs::File::open(&path)?;
        let mut buf = Vec::new();
        env_file.read_to_end(&mut buf)?;
        let env = Env::deserialize(&mut Deserializer::new(&*buf))?;
        Ok(env)
    }

    pub fn base_dir() -> Result<PathBuf, anyhow::Error> {
        let runtime_dir =
            std::env::var("XDG_RUNTIME_DIR").context("unable to read $XDG_RUNTIME_DIR")?;
        let mut dir = PathBuf::from(runtime_dir);
        dir.push("protonhax-rs");
        Ok(dir)
    }
}
