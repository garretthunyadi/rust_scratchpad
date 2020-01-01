use futures::executor::block_on;
use std::thread::sleep;
use std::time::Duration;
pub fn main() {
    let o = pre_retrieval();
    block_on(o);
}
async fn pre_retrieval() {
    let o = retrieve_val().await;
    println!("{:?}", o);
}
async fn retrieve_val() -> i32 {
    // Simulates some delays, e.g., network latency
    let ten_secs = Duration::from_secs(3);
    sleep(ten_secs);

    // return some value
    5
}
