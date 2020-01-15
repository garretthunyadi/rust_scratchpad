extern crate url;
use url::Url;

pub async fn main() {
    println!("wappalyzer_crate");
    let url = Url::parse(&String::from("http://google.com")).unwrap();
    let res = wappalyzer::scan(url).await;
    println!("{:?}", res);
}
