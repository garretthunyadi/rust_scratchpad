type Domain = String;

///  Scan states that consume the request to produce a result
/// ContentScan -> CoreScan
///  DnsScan -> ContentScan -> CoreScan
///
use rand::Rng;
pub fn main() {
    let res = match scan_states() {
        Ok(res) => res.techs.join(","),
        Err(e) => match e {
            ScanError::Core(s) => format!("Core Scan Error: {}", s),
            ScanError::Dns(s) => format!("Dns Scan Error: {}", s),
            _ => String::from("other error"),
        },
    };
    println!("{}", res);
}

pub fn scan_states() -> Result<CoreScanResult, ScanError> {
    let dns_req = DnsScanRequest::Domain("www.google.com".to_string());
    let dns = dns_req.scan()?;
    let head_scan: HeadScanResult = dns.scan()?;
    println!("{}", head_scan.content_length);

    // alt path
    let dns_req = DnsScanRequest::Domain("www.google.com".to_string());
    let dns = dns_req.scan()?;
    let content_scan: ContentScanResult = dns.scan()?;
    let core = content_scan.scan()?;
    println!("{:?}", core.techs);

    Ok(core)
}
struct Domain2(String);
struct Host(String);

pub enum ScanError {
    Dns(String),
    Content(String),
    Core(String),
    Head(String),
}
enum DnsScanRequest {
    Domain(String),
    DomainAndHost(String, String),
}
struct DnsScanResult {
    domain: String,
    host: String,
}
struct HeadScanResult {
    domain: String,
    content_length: u64,
}
enum ContentScanRequest {
    Domain(String),
    DomainAndHost(String, String),
    DnsScan(DnsScanResult),
}
struct ContentScanResult {
    domain: String,
    content: String,
}

enum CoreScanRequest {
    Domain(String),
    DomainAndHost(Domain, Host),
    DnsScan(DnsScanResult),
}
pub struct CoreScanResult {
    pub domain: String,
    pub techs: Vec<String>,
}

trait Scanner<T> {
    fn scan(self) -> Result<T, ScanError>;
}
impl Scanner<DnsScanResult> for DnsScanRequest {
    fn scan(self) -> Result<DnsScanResult, ScanError> {
        match self {
            DnsScanRequest::Domain(domain) => {
                let mut rng = rand::thread_rng();
                if rng.gen() {
                    Ok(DnsScanResult {
                        domain,
                        host: "a host".to_string(),
                    })
                } else {
                    Err(ScanError::Dns("unknown dns failure".to_string()))
                }
            }
            DnsScanRequest::DomainAndHost(domain, host) => Ok(DnsScanResult {
                domain,
                host: "a host".to_string(),
            }),
        }
    }
}
impl Scanner<HeadScanResult> for DnsScanResult {
    fn scan(self) -> Result<HeadScanResult, ScanError> {
        head_scan(self.domain)
    }
}
impl Scanner<ContentScanResult> for DnsScanResult {
    fn scan(self) -> Result<ContentScanResult, ScanError> {
        content_scan(self.domain)
    }
}
impl Scanner<CoreScanResult> for ContentScanResult {
    fn scan(self) -> Result<CoreScanResult, ScanError> {
        Ok(core_from_content(self.domain, self.content))
    }
}

fn head_scan(domain: Domain) -> Result<HeadScanResult, ScanError> {
    Ok(HeadScanResult {
        domain,
        content_length: 44,
    })
}
fn content_scan(domain: Domain) -> Result<ContentScanResult, ScanError> {
    Ok(ContentScanResult {
        domain,
        content: "This is my website.  It's <bold>pretty</bold>.".to_string(),
    })
}
fn core_from_content(domain: Domain, content: String) -> CoreScanResult {
    CoreScanResult {
        domain,
        techs: vec!["wordpress".to_string()],
    }
}
