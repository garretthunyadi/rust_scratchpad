use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub fn main() -> std::io::Result<()> {
    println!("threads_1");

    let (tx, rx) = channel();

    let tx1 = thread::spawn(move || {
        tx.send(1).expect("sender panicked");
        tx.send(2).expect("sender panicked");
        thread::sleep(Duration::from_secs(3));
    });

    let res = rx.recv().expect("receiver panicked");
    println!("Rec'd: {}", res);
    let res = rx.recv().expect("receiver panicked");
    println!("Rec'd: {}", res);
    // let res = rx.recv().expect("receiver panicked");
    // println!("Rec'd: {}", res);
    Ok(())
}
