fn connect() {
}

pub fn go() {
    let dns = DnsScanRequest::Domain("www.google.com".to_string());
    let head_scan = scan(dns);
}
struct Domain2(String);
struct Host(String);
enum DnsScanRequest {
    Domain(String),
    DomainAndHost(String,String),
}
struct DnsScanResult {
    domain: String,
    host: String,
}
enum ContentScanRequest {
    Domain(String),
    DomainAndHost(String,String),
    DnsScan(DnsScanResult)
}
enum CoreScanRequest {
    Domain(String),
    DomainAndHost(Domain,Host),
    DnsScan(DnsScanResult)
}
trait Scanr<T,U> {
    fn scan(t:T) -> U;
}