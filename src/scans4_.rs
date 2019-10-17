use std::io::{Error, ErrorKind};

// domain = Domain::from("google.com");
pub fn main() -> Result<()> {
    println!("Scans4");
    Ok(())
    // Err(ScanError::Other(String::from("Bogus Error")))
    // Err(ScanError::Unreachable)
}

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

type Result<T> = std::result::Result<T, ScanError>;
