// recursive types in a linked list
pub fn main() -> std::io::Result<()> {
    println!("boxes");

    struct Foo {
        bar: usize,
    }

    impl Foo {
        fn baz(&self) -> usize {
            self.bar + 10
        }
    }

    let fooo = Foo { bar: 0 };
    assert_eq!(fooo.baz(), 10);

    let boxed = Box::new(fooo);
    assert_eq!(boxed.baz(), 10);

    Ok(())
}

enum List {
    Cons(Box<List>),
    Nil,
}
use crate::rust_book::boxes::List::{Cons, Nil};

impl List {
    fn lenn(&self) -> usize {
        match self {
            Cons(bx) => 1 + bx.lenn(),
            Nil => 0,
        }
    }
}

#[test]
fn test_ll() {
    let l0 = List::Nil;
    let l1 = List::Cons(Box::new(List::Nil));
    let l2 = List::Cons(Box::new(List::Cons(Box::new(List::Nil))));
    assert_eq!(l0.lenn(), 0);
    assert_eq!(l1.lenn(), 1);
    assert_eq!(l2.lenn(), 2);
}

struct MyBox<T>(T);
impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

#[test]
fn test_my_box() {
    let my_box = MyBox(13);
    assert_eq!(*my_box, 13);

    let mut my_box = MyBox(13);
    my_box.0 = 14;
    assert_eq!(*my_box, 14);
}
