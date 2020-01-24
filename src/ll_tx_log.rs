use std::cell::RefCell;
use std::rc::Rc;

pub fn main() {
    println!("linked list/transaction log");

    let mut log = TxLog::new_empty();
    assert_eq!(log.pop_head(), None);

    log.append_to_end(1);
    assert_eq!(log.pop_head(), Some(1));
    // assert_eq!(log.pop_head(), None);

    // log.append_to_end(2);
    // assert_eq!(log.pop_head(), Some(2));
    // assert_eq!(log.pop_head(), None);

    // log.append_to_end(3);
    // log.append_to_end(4);
    // assert_eq!(log.pop_head(), Some(3));
    // assert_eq!(log.pop_head(), Some(4));
    // assert_eq!(log.pop_head(), None);
}

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    val: i32,
    next: SingleLink,
}
impl Node {
    fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, next: None }))
    }
}

struct TxLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}
impl TxLog {
    pub fn new_empty() -> TxLog {
        TxLog {
            head: None,
            tail: None,
            length: 0,
        }
    }
    pub fn append_to_end(&mut self, val: i32) {
        let new = Node::new(val);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        }
        self.length += 1;
        self.tail = Some(new);
    }

    #[allow(clippy::option_map_unit_fn)]
    pub fn pop_head(&mut self) -> Option<i32> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is wrong.")
                .into_inner()
                .val
        })
    }
    // pub fn append_to_top(&mut self, val: i32) {
    //     let new_link = Node::new(val);
    //     if self.head.is_none() {
    //         assert!(self.tail.is_none());
    //         // head and tail should be set to the new value
    //         self.head = Some(new_link.clone());
    //         self.tail = Some(new_link.clone());
    //     } else {
    //         assert!(self.tail.is_some());
    //         // head is replaced with a new head, contining
    //         // the new val and pointing to the old head.
    //         // the tail is unchanged
    //         new_link.borrow_mut().next = self.head.clone();
    //         // let prev_head = self.head.clone();
    //         // new_link.unwrap().borrow_mut().next = prev_head;
    //         self.head = new_link;
    //     }
    // }
    // pub fn pop_from_bottom(&mut self) -> Option<i32> {
    //     None
    // }
}

// pub fn main() {
//     println!("linked list");
//     let mut head = Link { val: 4, next: None };
//     let next = Link { val: 5, next: None };
//     head.next = Some(Rc::new(RefCell::new(next)));
//     assert_eq!(head.val, 4);
//     assert_eq!(head.next.unwrap().borrow().val, 5);

//     // head.next.unwrap().borrow_mut().next = Some(Rc::new(RefCell::new(Link{val:6,next:None})));
//     let mut head = Link::new(11);
//     head.attach(12);
//     assert_eq!(head.val, 11);

//     assert_eq!(head.next().unwrap().borrow().val, 12);
//     head.attach(13);

//     //
//     //
//     println!("const log");
//     let mut list: List<u32> = List::new();
//     list.push(0);
//     assert_eq!(*list.at(0), 0);
//     list.push(1);
//     assert_eq!(*list.at(1), 1);
//     list.push(2);
//     assert_eq!(*list.at(2), 2);
// }

// pub struct List<T> {
//     head: Option<Link<T>>,
//     // tail: Option<Link<T>>,
// }
// impl<T> List<T> {
//     pub fn new() -> List<T> {
//         List { head: None }
//     }
//     pub fn push(&mut self, val: T) {
//         self.head = match &self.head {
//             None => Some(Link::new(val)),
//             Some(link) => Some(Link::new_with_next_link(val, link.next.clone())),
//         }
//     }
//     pub fn at(&self, i: usize) -> &T {
//         let mut curr = self.head.or(None);
//         for _ in 0..i {
//             if curr.is_none() {
//                 break;
//             } else {
//                 curr = curr.clone().unwrap().next;
//             }
//         }
//         let x = &self.head;
//         let y = x.as_ref().unwrap();
//         y.val()
//     }
// }
// type Next<T> = Option<Rc<RefCell<Link<T>>>>;

// pub struct Link<T> {
//     val: T,
//     next: Next<T>,
// }

// impl<T> Link<T> {
//     pub fn new(val: T) -> Self {
//         Self { val, next: None }
//     }
//     pub fn new_with_next_link(val: T, next: Next<T>) -> Self {
//         Self { val, next }
//     }
//     pub fn val(&self) -> &T {
//         &self.val
//     }
//     pub fn next(&self) -> Next<T> {
//         self.next.clone()
//     }
//     pub fn attach(&mut self, val: T) {
//         let mut new_node = Link::new(val);
//         new_node.next = match &self.next {
//             Some(rc) => Some(rc.clone()),
//             None => None,
//         };
//         self.next = Some(Rc::new(RefCell::new(new_node)));
//     }
//     pub fn push(&mut self, val: T) {
//         self.attach(val)
//     }
// }
