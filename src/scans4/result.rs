use std::io::{Error, ErrorKind};

// ensure valid domain
#[derive(Debug)]
pub enum ScanError {
    Unreachable,
    InvalidDomain,
    IoError(std::io::Error),
    Other(String),
}

impl From<ScanError> for std::io::Error {
    fn from(err: ScanError) -> Self {
        match err {
            ScanError::Unreachable => Error::new(ErrorKind::Other, "Unreachable"),
            ScanError::InvalidDomain => Error::new(ErrorKind::Other, "InvalidDomain"),
            ScanError::IoError(err) => err,
            ScanError::Other(details) => {
                Error::new(ErrorKind::Other, format!("OtherScanError: {}", details))
            }
        }
    }
}

impl From<std::io::Error> for ScanError {
    fn from(error: std::io::Error) -> Self {
        ScanError::IoError(error)
    }
}

pub type Result<T> = std::result::Result<T, ScanError>;
