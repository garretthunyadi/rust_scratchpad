use std::io::{Error, ErrorKind};

// ensure valid domain
pub enum ScanError {
    Unreachable,
    Other(String),
}

impl From<ScanError> for std::io::Error {
    fn from(err: ScanError) -> Self {
        match err {
            ScanError::Unreachable => Error::new(ErrorKind::Other, "Unreachable"),
            ScanError::Other(details) => {
                Error::new(ErrorKind::Other, format!("OtherScanError: {}", details))
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, ScanError>;
