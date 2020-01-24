// [x] Display list
// [x] Keep a ref to the tail as an optimization
// [x] Functionality to change values in the LL

// #![feature(test)]
// extern crate test;

use std::cell::RefCell;
use std::rc::Rc;

pub fn main() {
    puts!("Linked List");

    let mut list = LL::new(10);
    println!("{}", list);
    list.add(20);
    println!("{}", list);
    list.add(30);
    println!("{}", list);

    let iter = LLIterator::new(&list);
    for i in iter {
        println!("- {}", i);
    }

    // mutate
    list.nth(1).unwrap().val = 21;
    println!("{:?}", list.values());
    LL::set(&mut list.head.clone().unwrap(), 11);
    println!("{:?}", list.values());
}

// A link is differs from a node as the link has
// the option+ref counting+ ref cell mechanisms.
type Link<T> = Rc<RefCell<Node<T>>>;

#[derive(Clone)]
struct Node<T: Copy> {
    val: T,
    next: Option<Link<T>>,
}
impl<T: Copy> Node<T> {
    fn new(val: T) -> Node<T> {
        Node { val, next: None }
    }

    // helper - 0-based
    fn nth(&self, n: u32) -> Option<Node<T>> {
        if n == 0 {
            return Some(self.clone());
        }

        let mut curr = self.next.clone();
        for _ in 1..n {
            if let Some(node) = curr.clone() {
                curr = node.borrow().next.clone();
            } else {
                return None;
            }
        }

        Some(curr.unwrap().borrow().clone())
    }

    // helper
    fn new_link(val: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node::new(val)))
    }
}

struct LL<T: Copy> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
}
impl<T: Copy> LL<T> {
    fn new(val: T) -> LL<T> {
        let link = Some(Node::new_link(val));
        LL {
            head: link.clone(),
            tail: link,
        }
    }

    fn add(&mut self, val: T) {
        assert!(self.head.is_some());
        assert!(self.tail.is_some());

        let tail = self.tail.clone().unwrap();
        let mut tail = tail.borrow_mut();

        let new_link = Some(Node::new_link(val));
        tail.next = new_link.clone();
        // update the tail pointer
        self.tail = new_link;
    }

    // helper
    fn next(link: Link<T>) -> Option<Link<T>> {
        link.borrow().next.clone()
    }

    // helper
    fn nth(&self, n: u32) -> Option<Node<T>> {
        let link;
        if let Some(head) = &self.head {
            link = Node::nth(&head.borrow(), n)
        } else {
            link = None
        }
        link
    }

    // helper
    fn values(&self) -> Vec<T> {
        let mut vals = vec![];
        let mut link = self.head.clone();
        while link.is_some() {
            vals.push(link.clone().unwrap().borrow().val);
            link = LL::next(link.unwrap());
        }
        vals
    }

    // helper
    fn set(link: &mut Link<T>, val: T) {
        link.borrow_mut().val = val;
    }
}

impl<T: Copy + std::fmt::Debug> std::fmt::Display for LL<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.values())
    }
}

struct LLIterator<'a, T: Copy> {
    ll: &'a LL<T>,
    curr: Option<Link<T>>,
}
impl<'a, T: Copy> LLIterator<'a, T> {
    fn new(ll: &'a LL<T>) -> LLIterator<T> {
        LLIterator {
            ll,
            curr: ll.head.clone(),
        }
    }
    fn value(&self) -> Option<T> {
        match &self.curr {
            Some(cell) => Some(cell.borrow().val),
            None => None,
        }
    }
}

impl<'a, T: Copy> Iterator for LLIterator<'a, T> {
    // type Item = Rc<RefCell<Node<T>>>;
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let val = match &self.curr {
            Some(_) => {
                // self.curr = cell.borrow().next;
                self.value()
            }
            None => None,
        };

        // we have some value, therefore there is something following it
        // in the list (though this coould be a 'None' option). Now we 'advance'
        // the curr pointer
        if val.is_some() {
            let nxt = self.curr.clone().unwrap().borrow().next.clone();
            self.curr = nxt;
        }
        val
    }
}

impl<'a, T: Copy> std::ops::Deref for LLIterator<'a, T> {
    type Target = LL<T>;

    fn deref(&self) -> &Self::Target {
        &self.ll
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ll_basic() {
        let mut ll = LL::new(111);
        assert_eq!(ll.values(), vec![111]);
        ll.add(222);
        assert_eq!(ll.values(), vec![111, 222]);
        ll.add(333);
        assert_eq!(ll.values(), vec![111, 222, 333]);
    }

    #[test]
    fn test_nth() {
        let mut first = Node::new(11);
        let second = Node::new_link(22);
        first.next = Some(second.clone());
        let third = Node::new_link(33);
        second.borrow_mut().next = Some(third);

        assert_eq!(first.nth(0).unwrap().val, 11);
        assert_eq!(first.nth(1).unwrap().val, 22);
        assert_eq!(first.nth(2).unwrap().val, 33);
    }

    #[test]
    fn iter_basic() {
        let mut ll = LL::new(111);
        ll.add(222);
        ll.add(333);
        let mut iter = LLIterator::new(&ll);
        assert_eq!(iter.next(), Some(111));
        assert_eq!(iter.next(), Some(222));
        assert_eq!(iter.next(), Some(333));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mutation() {
        let mut ll = LL::new(111);
        ll.add(222);
        ll.add(333);
        let mut iter = LLIterator::new(&ll);
        iter.next();
        assert_eq!(iter.value(), Some(222));
        LL::set(&mut iter.curr.unwrap(), 2222);
        let mut iter = LLIterator::new(&ll);
        iter.next();
        assert_eq!(iter.value(), Some(2222));
    }
}
