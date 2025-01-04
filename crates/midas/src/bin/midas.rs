use clap::Parser;
use midas_cli::{Cli, Command};
use miette::*;

fn main() -> Result<()> {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
    match *cli.command {
        Command::Help(_) => {
            println!("Help command");
        }
    }
    Ok(())
}
