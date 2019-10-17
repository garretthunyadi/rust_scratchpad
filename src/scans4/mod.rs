mod result;

use result::{Result, ScanError};
use std::str::FromStr;

// domain = Domain::from("google.com");
pub fn main() -> Result<()> {
    println!("Scans 4");
    // let domain = "google.com".parse::<Domain>();
    let domain = Domain::from_str("google.codm")?;
    let mx: MxScanResult = domain.scan();
    println!("MxScanResult -> {:?}", mx);
    let dns: DnsScanResult = domain.scan();
    println!("DnsScanResult -> {:?}", dns);
    Ok(())
}

pub trait Scanner<ScanResult> {
    fn scan(&self) -> ScanResult;
}

#[derive(Debug, PartialEq)]
struct Domain {
    domain: String,
}

impl FromStr for Domain {
    type Err = ScanError;

    fn from_str(s: &str) -> std::result::Result<Self, ScanError> {
        // TODO: Tmp nieve version
        if s.contains(".com") {
            Ok(Domain {
                domain: String::from(s),
            })
        } else {
            Err(ScanError::InvalidDomain)
        }
    }
}

type Host = String;
type IP = String;

#[derive(Debug)]
struct MxScanResult {
    servers: Vec<Host>,
}

#[derive(Debug)]
struct DnsScanResult {
    ip: IP,
}

impl Scanner<MxScanResult> for Domain {
    fn scan(&self) -> MxScanResult {
        MxScanResult { servers: vec![] }
    }
}

impl Scanner<DnsScanResult> for Domain {
    fn scan(&self) -> DnsScanResult {
        DnsScanResult {
            ip: String::from(""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_parsing() {
        assert_eq!(
            Domain::from_str("google.com").unwrap(),
            Domain {
                domain: String::from("google.com")
            }
        );
        match Domain::from_str("google.bad") {
            Err(ScanError::InvalidDomain) => (),
            Err(err) => {
                panic!("unexpected type of error: {:?}", err);
            }
            Ok(_) => {
                panic!("unexpected successful parse");
            }
        }
        match Domain::from_str("bad") {
            Err(ScanError::InvalidDomain) => (),
            Err(err) => {
                panic!("unexpected type of error: {:?}", err);
            }
            Ok(_) => {
                panic!("unexpected successful parse");
            }
        }
    }
}
