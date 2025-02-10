use std::process::ExitCode;

use miette::*;

fn main() -> Result<ExitCode> {
    midas::main(std::env::args_os())
}
