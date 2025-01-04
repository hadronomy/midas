use std::ffi::OsString;
use std::process::ExitCode;

use clap::Parser;
use clap::error::{ContextKind, ContextValue};
use midas_cli::{Cli, Command};
use miette::*;
use owo_colors::OwoColorize;

pub mod error;

#[cfg(feature = "deno")]
pub mod typescript;

pub fn main<Args, T>(args: Args) -> ExitCode
where
    Args: Iterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let cli = match Cli::try_parse_from(args) {
        Ok(cli) => cli,
        Err(err) => err.exit(),
    };
    let result: Result<ExitCode> = match *cli.command {
        Command::Help(_) => Err(miette!("Help command is not implemented yet.")),
    }
    .wrap_err("Failed to execute command");
    match result {
        Ok(exit_code) => exit_code,
        Err(err) => {
            let mut causes = err.chain();
            eprintln!("{} {}", "error:".red().bold(), causes.next().unwrap().to_string().trim());
            for err in causes {
                eprintln!("  {} {}", "Caused by:".red().bold(), err.to_string().trim());
            }
            ExitStatus::Error.into()   
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) enum ExitStatus {
    /// The command succeeded.
    Success,

    /// The command failed due to an error in the user input.
    Failure,

    /// The command failed with an unexpected error.
    Error,

    /// The command's exit status is propagated from an external command.
    External(u8),
}

impl From<ExitStatus> for ExitCode {
    fn from(status: ExitStatus) -> Self {
        match status {
            ExitStatus::Success => Self::from(0),
            ExitStatus::Failure => Self::from(1),
            ExitStatus::Error => Self::from(2),
            ExitStatus::External(code) => Self::from(code),
        }
    }
}
