use std::process::ExitCode;

fn main() -> ExitCode {
    midas::main(std::env::args_os())
}
