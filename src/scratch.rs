use std::cell::RefCell;
use std::rc::Rc;

pub fn ref_cell() {
    let x = RefCell::new(5);
    let y = &x;
    *y.borrow_mut() = 6;
    println!("{}",x.borrow());
}
pub fn rc() {

    // let owner1 = Rc::new(RefCell::new(4));
    // println!("{}", owner1.);
    // let owner2 = Rc::clone(&owner1);

    // take_rc(owner1);
    // let owner3 = Rc::clone(&owner2);
    // take_rc(owner2);
}

fn take_rc(rc: Rc<i32>) {}
