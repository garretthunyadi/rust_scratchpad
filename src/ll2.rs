use std::cell::RefCell;
use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;

pub fn main() -> std::io::Result<()> {
    println!("ll2");

    let mut llist: LinkedList<String> = LinkedList::new();
    let list: &mut dyn StackPlus<_> = &mut llist;
    assert_eq!(list.len(), 0);

    list.push(s!("first"));
    assert_eq!(list.len(), 1);

    list.push(s!("second"));
    assert_eq!(list.len(), 2);

    list.push(s!("third"));
    assert_eq!(list.len(), 3);

    let val = list.pop();
    assert_eq!(list.len(), 2);
    assert_eq!(val, Some(s!("third")));

    println!("{}", llist);
    Ok(())
}

trait StackPlus<T: Clone> {
    fn push(&mut self, v: T);
    // fn prepend(&mut self, v: T);
    fn pop(&mut self) -> Option<T>;
    fn pop_from_end(&mut self) -> Option<T>;
    fn len(&self) -> usize;
}

type Link<T> = Rc<RefCell<Node<T>>>;

struct Node<T> {
    val: T,
    next: Option<Link<T>>,
}
impl<T> Node<T> {
    fn new(val: T) -> Node<T> {
        Node { val, next: None }
    }
    fn new_with_next(val: T, next: Option<Link<T>>) -> Node<T> {
        Node { val, next }
    }
}

struct LinkedList<T> {
    head: Option<Link<T>>,
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList { head: None }
    }
    fn replace_at(&mut self, index: usize, v: T) {
        unimplemented!();
    }
}
impl<T: Display + Clone> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut v = vec![];
        for l in self {
            v.push(l.to_string());
        }
        write!(f, "{}", v.join(" -> "))
    }
}

impl<T: Clone> IntoIterator for &LinkedList<T> {
    type Item = T;
    type IntoIter = LLIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        LLIter {
            curr: self.head.clone(),
        }
    }
}

impl<T: Clone> StackPlus<T> for LinkedList<T> {
    fn push(&mut self, v: T) {
        self.head = match &self.head {
            None => Some(Rc::new(RefCell::new(Node::new(v)))),
            Some(rc_node) => Some(Rc::new(RefCell::new(Node::new_with_next(
                v,
                Some(rc_node.clone()),
            )))),
        };
    }

    fn pop(&mut self) -> Option<T> {
        let (v, h) = match &self.head {
            Some(head) => {
                let hd = head.borrow();
                let val = hd.val.clone();
                (Some(head.borrow().val.clone()), hd.next.clone())
            }
            None => (None, None),
        };

        self.head = h;
        v
    }
    fn pop_from_end(&mut self) -> Option<T> {
        None
    }

    fn len(&self) -> usize {
        self.into_iter().count()
    }

    // fn last(&self) -> Option<T> {
    //     None
    // }
    // fn append(&mut self, v: T) {}
    // fn pop_from_end(&mut self) -> Option<T> {
    //     None
    // }
}

#[test]
fn linked_list() {
    let mut llist: LinkedList<usize> = LinkedList::new();
    let list: &mut dyn StackPlus<_> = &mut llist;
    assert_eq!(list.len(), 0);
    // assert!(list.first().is_none());
    // assert!(list.last().is_none());

    list.push(1);
    assert_eq!(list.len(), 1);
    // assert_eq!(list.first(), Some(1));
    // assert_eq!(list.last(), Some(1));

    list.push(2);
    assert_eq!(list.len(), 2);
    // assert_eq!(list.first(), Some(1));
    // assert_eq!(list.last(), Some(2));

    list.push(3);
    assert_eq!(list.len(), 3);
    // assert_eq!(list.first(), Some(1));
    // assert_eq!(list.last(), Some(3));

    let t = list.pop();
    assert_eq!(list.len(), 2);

    let t = list.pop_from_end();
    let t = list.pop();
    assert_eq!(list.len(), 1);
}

#[test]
fn to_string() {
    let mut ll = LinkedList::new();
    ll.push(s!("one"));
    ll.push(s!("two"));
    ll.push(s!("three"));

    // ll.replace_at(1, S!("two/mod"));
    assert_eq!(ll.to_string(), s!("three -> two -> one"));
}

#[test]
fn replace() {
    let mut ll = LinkedList::new();
    ll.push(s!("one"));
    ll.push(s!("two"));
    ll.push(s!("three"));

    ll.replace_at(1, s!("two/mod"));
    assert_eq!(ll.to_string(), s!("three -> /mod -> one"));
}

struct LLIter<T> {
    curr: Option<Link<T>>,
}
impl<T: Clone> Iterator for LLIter<T> {
    type Item = T;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        let (maybe_val, new_curr) = if let Some(curr) = &self.curr {
            let val = curr.borrow().val.clone();
            (Some(val), curr.borrow().next.clone())
        } else {
            (None, None)
        };

        self.curr = new_curr;
        maybe_val
    }
}

#[test]
fn ll_iter() {
    let mut list: LinkedList<usize> = LinkedList::new();
    list.push(11);
    list.push(22);

    let mut iter = LLIter {
        curr: list.head.clone(),
    };

    assert_eq!(iter.next(), Some(22));
    assert_eq!(iter.next(), Some(11));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}
