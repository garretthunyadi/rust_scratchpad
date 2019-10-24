use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub fn main() -> std::io::Result<()> {
    threads_and_channels_1()
}
#[test]
fn what_is_allowed_to_move_to_a_thread() {
    // First, if we don't add the 'move' keyword
    // let x = 1;
    // // closure may outlive the current function, but it borrows `x`, which is owned by the current function
    // thread::spawn(|| {
    //     assert_eq!(x, 1);
    // });

    // A simple value can be moved
    let x = 1;
    thread::spawn(move || {
        assert_eq!(x, 1);
    });

    // A simple struct can be moved withouth fuss.
    #[derive(Debug, PartialEq)]
    struct Foo {}
    let x = Foo {};
    thread::spawn(move || {
        assert_eq!(x, Foo {});
    });
    // Now x is unavalable to us, it became owned by the thread.
    // assert_eq!(x, Foo {});

    // A vec of simple values can be moved too.
    let x = vec![1, 2, 3];
    thread::spawn(move || {
        assert_eq!(x, vec![1, 2, 3]);
    });
}

pub fn threads_and_channels_1() -> std::io::Result<()> {
    println!("threads_and_channels_1");

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

pub fn threads_and_channels_2() -> std::io::Result<()> {
    println!("threads_and_channels_2");

    let (tx, rx) = channel();

    let tx1 = thread::spawn(move || {
        let s = String::from("some value");
        tx.send(s).expect("sender panicked");
        // println!("{}", s); // compile error: s was moved

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
