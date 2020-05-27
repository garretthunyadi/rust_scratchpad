#[test]
fn rc2() {
    use std::rc::Rc;

    let mut rc;
    {
        let x = 5;
        rc = Rc::new(x)
    }
    assert_eq!(*rc, 5);
    // *rc = 6;
    let m = Rc::get_mut(&mut rc).unwrap();
    *m = 6;
    assert_eq!(*rc, 6);
}

#[test]
fn rc2_w_object() {
    struct Thing(u8, String);
    use std::rc::Rc;

    let y = Thing(44, "bar".to_string());

    let mut rc;
    {
        let x = Thing(55, "foo".to_string());
        rc = Rc::new(x)
    }
    assert_eq!((*rc).0, 55);
    // *rc = 6;
    {
        let m = Rc::get_mut(&mut rc).unwrap();
        // assert_eq!((*rc).0, 44);
    }
    let rc2 = rc.clone();
    // (*rc2).0 = 1;
    // *rc2 = y;

    // assert_eq!((*rc).0, 1); // TODO
    // assert_eq!((*rc2).0, 1); // TODO
}

// RefCell
#[test]
fn refcell2() {
    // use std::cell::Cell; // A mutable memory location.
    // use std::cell::Ref; // Wraps a borrowed reference to a value in a RefCell box. A wrapper type for an immutably borrowed value from a RefCell<T>.
    use std::cell::RefCell; // A mutable memory location with dynamically checked borrow rules

    let mut refcell;
    {
        let x = 5;
        refcell = RefCell::new(x)
    }

    let m = refcell.get_mut();
    assert_eq!(*m, 5);

    // *rc = 6;
    *m = 6;
    assert_eq!(*m, 6);

    // assert_eq!(*refcell, 6); // type `std::cell::RefCell<{integer}>` cannot be dereferenced rustc(E0614)
    let r = refcell.borrow();
    assert_eq!(*r, 6);
}
