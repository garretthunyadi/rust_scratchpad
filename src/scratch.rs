use std::cell::RefCell;

pub fn main() {
    puts!("Scratch");

    // create and update a ref cell
    let rc = RefCell::new("This");
    {
        let mut x = rc.borrow_mut();
        *x = "That";
    }
    println!("->{}", rc.borrow());

    // given a vec of ref cells, swap the third and fifth one
    
}
