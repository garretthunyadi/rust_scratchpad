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

struct BatteryState<State> {
    log: Rc<RefCell<Vec<String>>>,
    state: State,
}

struct Idle();
struct Filling {
    percent_complete: u8,
}
struct Done();
impl BatteryState<Idle> {
    fn new() -> BatteryState<Idle> {
        BatteryState {
            log: Rc::new(RefCell::new(vec![])),
            state: Idle(),
        }
    }
}
// transitions
impl From<BatteryState<Idle>> for BatteryState<Filling> {
    fn from(s: BatteryState<Idle>) -> BatteryState<Filling> {
        BatteryState {
            log: s.log,
            state: Filling {
                percent_complete: 0,
            },
        }
    }
}

impl From<BatteryState<Filling>> for BatteryState<Done> {
    fn from(s: BatteryState<Filling>) -> BatteryState<Done> {
        BatteryState {
            log: s.log,
            state: Done(),
        }
    }
}

impl From<BatteryState<Done>> for BatteryState<Idle> {
    fn from(s: BatteryState<Done>) -> BatteryState<Idle> {
        BatteryState {
            log: s.log,
            state: Idle(),
        }
    }
}

#[test]
fn transitions() {
    let idle = BatteryState::<Idle>::new();
    let idle = BatteryState::<Idle>::from(idle); // State from the same state works (not sure if this desired)
                                                 // let done = BatteryState::<Done>::from(Idle); // Error - YAY!
    let filling = BatteryState::<Filling>::from(idle);
    let done = BatteryState::<Done>::from(filling);
    let _idle = BatteryState::<Idle>::from(done);
}

fn to_filling(s: BatteryState<Idle>) -> BatteryState<Filling> {
    s.into()
}
fn to_done(s: BatteryState<Filling>) -> BatteryState<Done> {
    s.into()
}
fn to_idle(s: BatteryState<Done>) -> BatteryState<Idle> {
    s.into()
}

fn transition_functions() {
    let idle = BatteryState::<Idle>::new();
    // let done = BatteryState::<Done>::from(Idle); // Error - YAY!
    let filling = to_filling(idle);
    let done = to_done(filling);
    let _idle = to_idle(done);

    // this uses lifetimes to prevent double-use of states and
    // it makes error states unrepresentable!
}

// ==========================================
// ==========================================
// ==========================================
// ==========================================
//
//          SATELLITE
//
// ==========================================
// ==========================================
// ==========================================
// ==========================================

enum BatteryStateWrapper {
    Idle(BatteryState<Idle>),
    Filling(BatteryState<Filling>),
    Done(BatteryState<Done>),
}
enum BatteryEvent {
    PowerLevel(),
}
struct Satellite {
    battery1: BatteryStateWrapper,
    battery2: BatteryStateWrapper,
}
impl Satellite {}
