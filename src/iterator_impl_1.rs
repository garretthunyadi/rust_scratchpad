pub fn main() -> std::io::Result<()> {
    println!("iterator_impl_1");

    // desired behavior
    let mut c = Counter::new();
    print!("{} ", c.next().unwrap());
    print!("{} ", c.next().unwrap());
    print!("{} ", c.next().unwrap());

    // Note that this example of creating a
    // struct via it's internals shouldn't be possible outside of the package
    // We'll test this in iterator_impl_2.rs
    let mut c = Counter { count: 100 };

    // while this is a perfeftly reasonable interface for a counter,
    // we want the only interface to be the
    print!("{} ", c.inc()); // this shouldn't be allowed either
    let mut c = Counter { count: 1 };

    print!("{} ", c.inc()); // but this should be allowed.
    print!("{} ", c.next().unwrap());
    println!();

    Ok(())
}

struct Counter {
    count: usize,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
    fn inc(&mut self) -> usize {
        self.count += 1;
        self.count
    }
}
impl Iterator for Counter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        Some(self.inc())
    }
}
