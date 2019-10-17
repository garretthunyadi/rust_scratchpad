mod result;

use result::{Result,ScanError};

// domain = Domain::from("google.com");
pub fn main() -> Result<()> {
    println!("Scans4");
    // Ok(())
    Err(ScanError::Other(String::from("Bogus Error")))
    // Err(ScanError::Unreachable)
}

