use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Should only be called by Steam with "protonhax init %command%"
    Init {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,
    },
    /// Runs a command in the context of the given appid with proton
    Run {
        appid: u32,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,
    },
    /// Runs a command in the context of the given appid
    Exec {
        appid: u32,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,
    },
    /// Runs cmd.exe in the context of the given appid
    Cmd { appid: u32 },
    /// Lists all currently running games
    Ls,
}
