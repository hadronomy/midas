use std::process::ExitCode;

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
