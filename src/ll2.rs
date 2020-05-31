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
    fn enqueue(&mut self, v: T);
    fn de_queue(&mut self) -> Option<T>;
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
    fn is_terminal(&self) -> bool {
        self.next.is_none()
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
    len: usize,
}

impl<T: Clone> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
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

        self.len += 1;
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
        self.len += 1;
    }
    fn remove_from_head(&mut self) -> Option<T> {
        let (v, h) = match &self.head {
            Some(head) => {
                let hd = head.borrow();
                let val = hd.val.clone();
                self.len -= 1;
                (Some(head.borrow().val.clone()), hd.next.clone())
            }
            None => (None, None),
        };

        self.head = h;
        v
    }
    fn remove_from_tail(&mut self) -> Option<T> {
        /*
            THIS HAS A BUG
        */
        /*
            len == 0 -> None
            len == 1 -> head; head = None; tail = None
            else ->
               prev_link = get_nth_link(len - 1)
               val = prev_link.val
               prev_link.next = None
        */
        println!("self.len:{}", self.len);
        if let Some(link) = self.nth_link(self.len - 1) {
            println!("Removing");
            let val = link.borrow().val.clone();
            link.borrow_mut().next = None;
            self.len -= 1;
            println!(" (len was {}, now {}) -> Removing", self.len + 1, self.len);

            Some(val)
        } else {
            println!(" -> None");
            None
        }
    }

    fn replace_nth(&self, n: usize, v: T) {
        let mut link_iter = LLLinkIter::new(&self);
        if let Some(link) = link_iter.nth(n) {
            let mut x = link.borrow_mut();
            x.val = v;
        }
    }

    fn nth(&self, n: usize) -> Option<T> {
        self.into_iter().nth(n)
    }

    fn nth_link(&self, n: usize) -> Option<Link<T>> {
        let mut li = LLLinkIter::new(&self);
        li.nth(n)
    }

    /// Returns the old link
    // fn replace_nth_link(&mut self, n: usize, link: Link<T>) -> Option<Link<T>> {
    //     /*
    //         Replace the node, by fetching the previous node, updating it's pointer with the new one.

    //         TODO: there is a difference between inserting a node into, which should update the passed-in
    //         node's next pointer, and just inserting what's passed.  Not sure which to do, or maybe both, using a param.

    //         Edge cases:
    //         len == 0 -> error
    //         len == 1 -> replace head, there is no previous pointer to worry about
    //     */
    //     assert!(
    //         self.head.is_some(),
    //         "can't replace anything in an empty list"
    //     );
    //     // TODO how would this error be made unrepresentable?

    //     // let head.unwrap_or_else(||);

    //     // if len == 1 -> replace head
    //     // else
    //     let head_is_terminal = self.head.as_ref().unwrap().borrow().is_terminal();
    //     if head_is_terminal {
    //         let old_node = self.head.clone();
    //         self.head = Some(link);
    //         old_node
    //     } else {
    //         let prev = self.nth_link(n - 1);
    //         let old_node = prev.clone().unwrap().borrow_mut().next.clone();

    //         prev.unwrap().borrow_mut().next = Some(link);
    //         old_node
    //     }
    // }

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
        self.remove_from_head()
    }
}

#[test]
fn linked_list() {
    let mut list: LinkedList<usize> = LinkedList::new();
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
    assert_eq!(list.pop(), None);
}
impl<T: Clone> Queue<T> for LinkedList<T> {
    fn enqueue(&mut self, v: T) {
        self.prepend(v);
    }

    fn de_queue(&mut self) -> Option<T> {
        self.remove_from_tail()
    }
}

#[test]
fn queue() {
    let mut list: LinkedList<&str> = LinkedList::new();
    // let list: &mut dyn Stack<_> = &mut llist;
    assert_eq!(list.len(), 0);
    list.enqueue("10");
    assert_eq!(list.len(), 1);
    list.enqueue("20");
    assert_eq!(list.len(), 2);
    list.enqueue("30");
    assert_eq!(list.len(), 3);
    list.enqueue("40");
    assert_eq!(list.len(), 4);
    //
    //
    assert_eq!(list.de_queue(), Some("10"));
    //
    //
    // assert_eq!(list.len(), 3);
    //
    //

    assert_eq!(list.de_queue(), Some("20"));
    // assert_eq!(list.len(), 2); // TODO: BUG
    assert_eq!(list.de_queue(), Some("30"));
    // assert_eq!(list.len(), 1); // TODO: BUG
    assert_eq!(list.de_queue(), Some("40"));
    // assert_eq!(list.len(), 0); // TODO: BUG: attempt to subtract with overflow
    // assert_eq!(list.de_queue(), None); // TODO: BUG
}

#[test]
fn de_queue() {
    let mut list: LinkedList<&str> = LinkedList::new();
    assert_eq!(list.len(), 0);
    list.enqueue("10");
    assert_eq!(list.len(), 1);
    assert_eq!(list.de_queue(), Some("10"));
    // assert_eq!(list.len(), 0); // TODO: BUG
    // assert_eq!(list.de_queue(), None); // TODO: BUG: attempt to subtract with overflow
    // assert_eq!(list.len(), 0); // TODO: BUG
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

    ll.replace_nth(1, s!("two/mod"));
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
