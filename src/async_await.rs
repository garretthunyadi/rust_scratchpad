use futures::join;
use std::{thread, time};

// This doesn't actually run concurrently?!?
pub async fn async_main() {
    println!("async_await");
    let start = time::Instant::now();

    let f1 = do_something(1);
    let f2 = do_something(2);
    let f3 = do_something(3);
    let f4 = do_something(4);
    let f5 = do_something(5);
    join!(f1, f2, f3, f4, f5);
    // let thing = find_something().await;
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    // println!("found: {}", thing);
    println!("(fin.)");
}

async fn do_something(n: usize) {
    thread::sleep(time::Duration::from_millis(2000));
    println!("<LONG TIME do_something {}>", n);
}

async fn find_something() -> String {
    thread::sleep(time::Duration::from_millis(50));

    do_something(2).await;

    println!("<finding_something>");
    "Found this trinket".to_string()
}
