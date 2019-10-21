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

#[test]
fn test_misc() {
    // let x = vec!["Jill", "Jack", "Jane", "John"];
    // let y = x.clone().into_iter().collect::<Vec<_>>();
    // assert_eq!(y.len(), 4);

    // struct Person {
    //     name: String,
    // }
    // impl Person {
    //     fn new<S: Into<String>>(name: S) -> Person {
    //         Person { name: name.into() }
    //     }
    // }
    // let person = Person::new("Herman");
    // let person = Person::new("Herman".to_string());
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

//
//  IntoIter
//
struct Pixel {
    r: i8,
    g: i8,
    b: i8,
}

impl IntoIterator for Pixel {
    type Item = i8;
    type IntoIter = PixelIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        PixelIntoIterator {
            pixel: self,
            index: 0,
        }
    }
}

struct PixelIntoIterator {
    pixel: Pixel,
    index: usize,
}

impl Iterator for PixelIntoIterator {
    type Item = i8;
    fn next(&mut self) -> Option<i8> {
        let result = match self.index {
            0 => self.pixel.r,
            1 => self.pixel.g,
            2 => self.pixel.b,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

#[test]
fn test_pixel() {
    let p = Pixel {
        r: 54,
        g: 23,
        b: 74,
    };
    for component in p {
        println!("{}", component);
    }
}
