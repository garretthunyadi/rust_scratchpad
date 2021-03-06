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
use std::cell::RefCell;
use std::rc::Rc;

struct BFM<State> {
    log: Rc<RefCell<Vec<String>>>,
    state: State,
}
// impl<State> BFM<State> {
// fn new() -> BFM<State> {
//     BFM{
//         log: Rc::new(RefCell::new(vec![])),
//         state: Waiting()
//     }
// }
// }

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

    // this uses lifetimes to prevent double-use of states and
    // it makes error states unrepresentable!
}
