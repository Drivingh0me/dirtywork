use thiserror::Error;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Io error: {0}")]
    Io(#[from] io::Error),
    #[error("Parse Int Error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Try From Int Error: {0}")]
    TryIntErr(#[from] std::num::TryFromIntError),
    #[error("Failed to Allocate Memory Error: {0}")]
    FailedAlloc(#[from] std::collections::TryReserveError),
    #[error("Incorrect vector size")]
    VectorSize,
    #[error("Element out of bounds")]
    MtxBounds,
    #[error("Item Not Found")]
    ItemNotFound(String),
}

// Testing
