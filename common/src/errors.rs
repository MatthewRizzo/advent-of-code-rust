use std::num::ParseIntError;
use thiserror::Error;

pub type AdventResult<T> = std::result::Result<T, AdventErrors>;

#[derive(Error, Debug)]
pub enum AdventErrors {
    #[error("Error solving advent challenge")]
    AdventError(String),
    #[error("File does not exist error")]
    FileError(String),
    #[error("I/O error")]
    IOError(#[from] std::io::Error),
    #[error("String -> int error")]
    ParseError(#[from] ParseIntError),
}
