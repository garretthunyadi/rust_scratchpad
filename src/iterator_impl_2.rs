// In iterator_impl_1, we had the code inside the module and could create a

pub fn main() -> std::io::Result<()> {
    println!("iterator_impl_2");

    // now we don't have permission to access the internal state
    // let mut c = counter::Counter { count: 100 };
    // c.inc());

    // but we can use the exposed interface, counter::new + Iterator::next.
    let mut c = counter::Counter::new();
    print!("{} ", c.next().unwrap());
    print!("{} ", c.next().unwrap());
    print!("{} ", c.next().unwrap());
    println!();

    Ok(())
}

mod counter {
    pub struct Counter {
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
}
