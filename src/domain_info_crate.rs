use domain_info::{Domain, Scanner};

pub fn main() {
    println!("domain_info_crate");
    let res = Domain::from("google.com").unwrap().scan();
    println!("{:?}", res);
}
