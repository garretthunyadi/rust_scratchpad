// machine
// states
// transitions
// events

use std::io::Error;

/*

  1. The satellite model (Satellite)
  2. The satellite simulator (SatSim)
  3. The controller (threads & channels)


  The model is deternimistic and functional
  The simulator provides changes to the external environment
    - Should the sim know about the Sat? Seems reasonable, but the reason for the sim is to
    simulate the random events (simulating only this aspect of the actial satellite, like failure rates
    not a model of the satellite), not the behavior of the sat.
  The controller creates the environmen
     - Creates the channels, satellite and sim (giving channels), launches threads

     sat = Sat
     channel = create channels
     threads = create thread for sim
        SatSim creates threads for each subsystem
            - each subsystem on a different thread
              the simulator itself can own threads, so the controller can just create it and listen to the channel for the sumulator



  Sat only responds (no communication logic)
  Controller -> Sat
  Sim

*/

pub fn main() -> Result<(), Error> {
    let mut state = start_state();
    println!("state: {:?}", state);
    state = state.next();
    println!("state: {:?}", state);

    let mut machine = start();
    println!("machine: {:?}", machine.state());
    machine.next();
    println!("machine: {:?}", machine.state());

    // now use as iterator
    let xs = machine.take(5).collect::<Vec<_>>();
    println!("state: {:?}", xs);
    Ok(())
}
pub fn start() -> Machine {
    Machine::new()
}
pub fn start_state() -> State {
    State::State1
}

pub struct Machine(State);

impl Machine {
    pub fn new() -> Machine {
        Machine(State::State1)
    }
    pub fn state<'a>(&'a self) -> &'a State {
        &(self.0)
    }
}

impl Iterator for Machine {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = self.0.clone().next();
        Some(self.0.clone())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum State {
    State1,
    State2,
    State3,
}

impl State {
    fn start() -> State {
        State::State1
    }
    pub fn next(self) -> State {
        use State::*;
        match self {
            State1 => State2,
            State2 => State3,
            State3 => State1,
        }
    }
}

//////
////// ======================================================================================
//////
//////

// use std::io::Error;

// #[derive(Clone, Debug, PartialEq)]
// struct State1();

// #[derive(Debug)]
// struct State2(u8);
// impl From<State1> for State2 {
//     fn from(_: State1) -> State2 {
//         State2(11)
//     }
// }

// #[derive(Clone, Debug, PartialEq)]
// struct State3(bool);
// impl From<State2> for State3 {
//     fn from(_: State2) -> State3 {
//         State3(true)
//     }
// }

// fn start() -> State1 {
//     State1()
// }

// pub fn main() -> Result<(), Error> {
//     let s = start();
//     let s: State2 = s.into();
//     let s: State3 = s.into();

//     println!("{:?}", s);
//     Ok(())
// }

//////
////// ======================================================================================
//////
//////

// use std::io::Error;

// pub fn main() -> Result<(), Error> {
//     let mut state = start_state();
//     println!("state: {:?}", state);
//     state = state.next();
//     println!("state: {:?}", state);

//     let mut machine = start();
//     println!("machine: {:?}", machine.state());
//     machine.next();
//     println!("machine: {:?}", machine.state());

//     // now use as iterator
//     let xs = machine.take(5).collect::<Vec<_>>();
//     println!("state: {:?}", xs);
//     Ok(())
// }
// pub fn start() -> Machine {
//     Machine::new()
// }
// pub fn start_state() -> State {
//     State::State1
// }

// pub struct Machine(State);

// impl Machine {
//     pub fn new() -> Machine {
//         Machine(State::State1)
//     }
//     pub fn state<'a>(&'a self) -> &'a State {
//         &(self.0)
//     }
// }

// impl Iterator for Machine {
//     type Item = State;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.0 = self.0.clone().next();
//         Some(self.0.clone())
//     }
// }

// #[derive(Clone, Debug, PartialEq)]
// pub enum State {
//     State1,
//     State2,
//     State3,
// }

// impl State {
//     fn start() -> State {
//         State::State1
//     }
//     pub fn next(self) -> State {
//         use State::*;
//         match self {
//             State1 => State2,
//             State2 => State3,
//             State3 => State1,
//         }
//     }
// }
