use std::ffi::OsString;
use std::process::ExitCode;

use clap::Parser;
pub use error::Error;
use midas_cli::{Cli, Command};
use miette::*;
use owo_colors::OwoColorize;
use status::ExitStatus;

pub mod error;
pub(crate) mod status;

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

    let miette_hook = miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .break_words(false)
                .word_separator(textwrap::WordSeparator::AsciiSpace)
                .word_splitter(textwrap::WordSplitter::NoHyphenation)
                .build(),
        )
    }))
    .map_err(|err| Error::SetupError(err.into()));

    if let Err(err) = miette_hook {
        eprintln!("{} {}", "error:".red().bold(), err.to_string().trim());
        return ExitStatus::Error.into();
    }

    let result: Result<ExitCode> = match *cli.command {
        Command::Help(_) => Err(Error::Unimplemented),
    }
    .wrap_err("Failed to execute command");

    match result {
        Ok(exit_code) => exit_code,
        Err(err) => {
            eprintln!("{} {:?}", "error:".red().bold(), err);
            ExitStatus::Error.into()
        }
    }
}
