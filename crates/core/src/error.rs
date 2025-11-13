use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MccabreError {
    #[error("Failed to read file {path}: {source}")]
    FileRead { path: PathBuf, source: io::Error },

    #[error("Unsupported file type: {0}")]
    UnsupportedFileType(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Tokenization failed: {0}")]
    TokenizationError(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, MccabreError>;
