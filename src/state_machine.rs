// this uses lifetimes to prevent double-use of states and
// it makes error states unrepresentable!

use std::cell::RefCell;
use std::rc::Rc;

struct BFM<State> {
    log: Rc<RefCell<Vec<String>>>,
    state: State,
}

struct Waiting();
struct Filling {
    percent_complete: u8,
}
struct Done();
impl BFM<Waiting> {
    fn new() -> BFM<Waiting> {
        BFM {
            log: Rc::new(RefCell::new(vec![])),
            state: Waiting(),
        }
    }
}
// transitions
impl From<BFM<Waiting>> for BFM<Filling> {
    fn from(s: BFM<Waiting>) -> BFM<Filling> {
        BFM {
            log: s.log,
            state: Filling {
                percent_complete: 0,
            },
        }
    }
}

impl From<BFM<Filling>> for BFM<Done> {
    fn from(s: BFM<Filling>) -> BFM<Done> {
        BFM {
            log: s.log,
            state: Done(),
        }
    }
}

impl From<BFM<Done>> for BFM<Waiting> {
    fn from(s: BFM<Done>) -> BFM<Waiting> {
        BFM {
            log: s.log,
            state: Waiting(),
        }
    }
}

#[test]
fn transitions() {
    let waiting = BFM::<Waiting>::new();
    let waiting = BFM::<Waiting>::from(waiting); // State from the same state works (not sure if this desired)
                                                 // let done = BFM::<Done>::from(waiting); // Error - YAY!
    let filling = BFM::<Filling>::from(waiting);
    let done = BFM::<Done>::from(filling);
    let _waiting = BFM::<Waiting>::from(done);
}

fn to_filling(s: BFM<Waiting>) -> BFM<Filling> {
    s.into()
}
fn to_done(s: BFM<Filling>) -> BFM<Done> {
    s.into()
}
fn to_waiting(s: BFM<Done>) -> BFM<Waiting> {
    s.into()
}

fn transition_functions() {
    let waiting = BFM::<Waiting>::new();
    // let done = BFM::<Done>::from(waiting); // Error - YAY!
    let filling = to_filling(waiting);
    let done = to_done(filling);
    let _waiting = to_waiting(done);
}
