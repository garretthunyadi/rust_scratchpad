use std::cell::RefCell;
use std::rc::Rc;

pub fn main() -> std::io::Result<()> {
    println!("ll2");

    // let mut list = LinkedList::new();
    // list.prepend(1);
    // list.append(2);
    // let t = list.pop();
    // let t = list.pop_from_end();

    Ok(())
}

trait StackPlus<T> {
    // fn first(&self) -> Option<std::cell::Ref<T>>;
    // fn last(&self) -> Option<T>;
    fn push(&mut self, v: T);
    // fn prepend(&mut self, v: T);
    fn pop(&mut self) -> Option<T>;
    fn pop_from_end(&mut self) -> Option<T>;
    fn len(&self) -> usize;
}

struct Node<T> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}
impl<T> Node<T> {
    fn new(val: T) -> Node<T> {
        Node { val, next: None }
    }
    fn new_with_next(val: T, next: Option<Rc<RefCell<Node<T>>>>) -> Node<T> {
        Node { val, next }
    }
}

type Link<T> = Rc<RefCell<Node<T>>>;

struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList { head: None, len: 0 }
    }
}
impl<T> StackPlus<T> for LinkedList<T> {
    fn push(&mut self, v: T) {
        self.head = match &self.head {
            None => Some(Rc::new(RefCell::new(Node::new(v)))),
            Some(rc_node) => Some(Rc::new(RefCell::new(Node::new_with_next(
                v,
                Some(rc_node.clone()),
            )))),
        };
        self.len += 1;
    }
    // fn first(&self) -> Option<std::cell::Ref<T>> {
    //     match &self.head {
    //         None => None,
    //         Some(rc_node) => Some(rc_node.borrow()),
    //     }
    // }
    fn pop(&mut self) -> Option<T> {
        // let target = &self.head;
        // self.head = Some(*target.unwrap());
        // let target = target.unwrap();

        // if let Some(link) = &self.head {
        //     let x = link.borrow();
        //     let y = &*x;
        //     Some(y.val)
        // } else {
        //     None
        // }
        None
    }
    fn pop_from_end(&mut self) -> Option<T> {
        None
    }

    fn len(&self) -> usize {
        // let mut cnt = 0 as usize;
        // let mut position: &Option<Link<T>> = &self.head;
        // if let Some(node) = position {
        //     cnt += 1;

        //     let x = *(node.borrow());
        //     // let reff = &*node.borrow();
        //     // if reff.next.is_some() {
        //     //     position = &reff.next;
        //     // }

        //     // position = &reff.next;
        //     // &std::option::Option<&std::rc::Rc<std::cell::RefCell<ll2::Node<T>>>>
        //     // &std::option::Option<std::rc::Rc<std::cell::RefCell<ll2::Node<T>>>>
        // }

        // loop {}
        self.len
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
fn test_list() {
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
    // assert_eq!(list.len(), 2); // TODO

    let t = list.pop_from_end();
    let t = list.pop();
    // assert_eq!(list.len(), 1); // TODO
}
