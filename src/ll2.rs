use std::cell::RefCell;
use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;

pub fn main() -> std::io::Result<()> {
    println!("ll2");
    let mut list: LinkedList<String> = LinkedList::new();
    // let list: &mut dyn Stack<_> = &mut llist;
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
    println!("{}", list);
    Ok(())
}

//   ------------------------------------------------------------------
//
//      Stack   |   Queue   |   Link   |   Node
//
//
trait Stack<T: Clone> {
    fn push(&mut self, v: T);
    fn pop(&mut self) -> Option<T>;
}
trait Queue<T: Clone> {
    fn into(&mut self, v: T);
    fn out(&mut self) -> Option<T>;
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

//   ------------------------------------------------------------------
//
//      LinkedList
//
//
struct LinkedList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
}

impl<T: Clone> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
        }
    }
    fn prepend(&mut self, v: T) {
        self.head = match &self.head {
            None => Some(Rc::new(RefCell::new(Node::new(v)))),
            Some(rc_node) => Some(Rc::new(RefCell::new(Node::new_with_next(
                v,
                Some(rc_node.clone()),
            )))),
        };

        if self.tail.is_none() {
            self.tail = self.head.clone();
        }
    }
    fn append(&mut self, v: T) {
        self.tail = match &self.tail {
            None => Some(Rc::new(RefCell::new(Node::new(v)))),
            Some(rc_node) => {
                // 1. create a new tail node
                let new_tail = Some(Rc::new(RefCell::new(Node::new_with_next(v, None))));
                // 2. point the current tail's next pointer to the new node
                rc_node.borrow_mut().next = new_tail.clone();
                // 3. return the new node, making it the new tail
                new_tail
            }
        };

        if self.head.is_none() {
            self.head = self.tail.clone();
        }
    }
    fn remove_from_head(&mut self) -> Option<T> {
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
    fn remove_from_tail(&mut self) -> Option<T> {
        unimplemented!();
        // let (v, h) = match &self.head {
        //     Some(head) => {
        //         let hd = head.borrow();
        //         let val = hd.val.clone();
        //         (Some(head.borrow().val.clone()), hd.next.clone())
        //     }
        //     None => (None, None),
        // };

        // self.head = h;
        // v
    }

    fn replace_at(&self, index: usize, v: T) {
        let mut link_iter = LLLinkIter::new(&self);
        if let Some(link) = link_iter.nth(index) {
            let mut x = link.borrow_mut();
            x.val = v;
            // let m = Rc::get_mut(link);
        }
    }
    fn len(&self) -> usize {
        self.into_iter().count()
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
impl<T: Clone> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LLIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        LLIter {
            curr: self.head.clone(),
        }
    }
}

impl<T: Clone> Stack<T> for LinkedList<T> {
    fn push(&mut self, v: T) {
        self.prepend(v);
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
}

impl<T: Clone> Queue<T> for LinkedList<T> {
    fn into(&mut self, v: T) {
        self.prepend(v);
    }

    fn out(&mut self) -> Option<T> {
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
}

#[test]
fn linked_list() {
    let mut list: LinkedList<usize> = LinkedList::new();
    // let list: &mut dyn Stack<_> = &mut llist;
    assert_eq!(list.len(), 0);
    list.push(1);
    assert_eq!(list.len(), 1);
    list.push(2);
    assert_eq!(list.len(), 2);
    list.push(3);
    assert_eq!(list.len(), 3);
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.len(), 2);
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.len(), 1);
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop(), None);
}

#[test]
fn to_string() {
    let mut ll = LinkedList::new();
    ll.push(s!("one"));
    ll.push(s!("two"));
    ll.push(s!("three"));
    assert_eq!(ll.to_string(), s!("three -> two -> one"));
}

#[test]
fn replace() {
    let mut ll = LinkedList::new();
    ll.push(s!("one"));
    ll.push(s!("two"));
    ll.push(s!("three"));

    ll.replace_at(1, s!("two/mod"));
    assert_eq!(ll.to_string(), s!("three -> two/mod -> one"));
}

#[test]
fn append() {
    let mut ll = LinkedList::new();
    ll.append(s!("one"));
    ll.append(s!("two"));
    ll.append(s!("three"));
    assert_eq!(ll.to_string(), s!("one -> two -> three"));
}

//   ------------------------------------------------------------------
//
//        LLIter
//
//
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
//   ------------------------------------------------------------------
//
//        LLLinkIter
//
//

struct LLLinkIter<T> {
    curr: Option<Link<T>>,
    index: usize,
}
impl<T: Clone> LLLinkIter<T> {
    fn new(list: &LinkedList<T>) -> LLLinkIter<T> {
        LLLinkIter {
            curr: list.head.clone(),
            index: 0,
        }
    }
}
impl<T: Clone> Iterator for LLLinkIter<T> {
    type Item = Link<T>;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        let (maybe_val, new_curr) = if let Some(curr) = &self.curr {
            let val = curr.clone();
            (Some(val), curr.borrow().next.clone())
        } else {
            (None, None)
        };

        self.curr = new_curr;
        maybe_val
    }
}

#[test]
fn ll_link_iter() {
    let mut list: LinkedList<usize> = LinkedList::new();
    list.push(11);
    list.push(22);

    let mut iter = LLLinkIter::new(&list);

    assert_eq!(iter.next().unwrap().borrow().val, 22);
    assert_eq!(iter.next().unwrap().borrow().val, 11);
    assert!(iter.next().is_none());
}
