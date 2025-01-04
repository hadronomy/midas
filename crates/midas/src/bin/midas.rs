use std::process::ExitCode;

use midas::main as midas_main;

fn main() -> ExitCode {
    midas_main(std::env::args_os())
}
