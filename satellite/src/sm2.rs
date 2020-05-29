/*
 The main requirement of the state machine is to make error states unrepresentable.

 the state machine must have no possibility for run time errors.
  -> States need to be separate objects, consumed

  In the initial, and "pretty" version, there is a match on the state and the event and
  the match allows runtime combos that require a panic.  Therefore, we were able to
  "represent error states"
*/

/*
 State can either be generic or a trait.
 The state must be consumed

  The simplified system to model is:

  Satellite State is determined by the individual states of subcomponents,
  each having their unique state schema.


*/

/*
This is a situation where we have a design that is correct, but need to use
interior mutability in order to stub or log or similar.
*/
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

struct Foo {
    clicks: Cell<isize>,
    log: RefCell<Vec<String>>,
    rclog: Rc<RefCell<Vec<String>>>,
    arclog: Arc<RefCell<Vec<String>>>,
}
impl Foo {
    fn new() -> Foo {
        Foo {
            clicks: Cell::new(0),
            log: RefCell::new(vec![]),
            rclog: Rc::new(RefCell::new(vec![])),
            arclog: Arc::new(RefCell::new(vec![])),
        }
    }
    fn click(&self) {
        self.clicks.set(self.clicks.take() + 1);
        self.log
            .borrow_mut()
            .push(format!("clicked {} times.", self.clicks.get()));
        self.rclog
            .borrow_mut()
            .push(format!("clicked {} times.", self.clicks.get()));
        self.arclog
            .borrow_mut()
            .push(format!("clicked {} times.", self.clicks.get()));
    }
}

#[test]
fn test_interior_mutability_logging() {
    let x = Foo::new();
    assert_eq!(x.clicks.get(), 0);
    assert_eq!(x.log.borrow().len(), 0);
    assert_eq!(x.rclog.borrow().len(), 0);
    assert_eq!(x.arclog.borrow().len(), 0);

    x.click();
    assert_eq!(x.clicks.get(), 1);
    x.click();
    assert_eq!(x.clicks.get(), 2);

    assert_eq!(x.log.borrow().len(), 2);
    assert_eq!(x.rclog.borrow().len(), 2);
    assert_eq!(x.arclog.borrow().len(), 2);
}
