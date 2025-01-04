use std::path::PathBuf;

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(midas::io_error))]
    IoError(#[from] std::io::Error),

    #[error("Setup error: {0}")]
    #[diagnostic(code(midas::setup_error))]
    SetupError(miette::Report),

    #[error("Path not found: {0}")]
    #[diagnostic(code(midas::path_not_found))]
    PathNotFound(PathBuf),

    #[error("Invalid path format")]
    #[diagnostic(code(midas::invalid_path))]
    InvalidPathFormat,

    #[error("Unimplemented feature")]
    #[diagnostic(code(midas::unimplemented))]
    #[diagnostic(help("Ooops! This feature is not implemented yet."))]
    Unimplemented,

    #[error(transparent)]
    #[diagnostic(transparent)]
    UnknownError(#[from] UnknownError),
}

#[derive(Error, Diagnostic, Debug)]
#[error("another error")]
pub struct UnknownError {
    #[label("here")]
    pub at: SourceSpan,
}
