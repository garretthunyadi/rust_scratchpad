#![allow(dead_code)]

extern crate nom;

use std::io::Error;
use std::sync::mpsc;
mod proxy;
mod satellite;
mod satsim;
mod sm2;
mod sm3;
mod state_machine;

fn main() -> Result<(), Error> {
  // state_machine::main()?;
  // satellite::main()?;
  // satsim::main()?;

  // construct satsim
  // comm_tx: std::sync::mpsc::Sender<SatEvent>
  let (tx, rx) = mpsc::channel();

  let satsim = satsim::SatSim::new(tx);
  // construct sat, inject satellite trait

  let _sat = satellite::Satellite::new(&satsim);
  for event in rx.recv() {
    match event {
      satsim::SatEvent::BatteryStateChange(_part_id, _percent) => {
        // let state = sat.handle_event(satellite::Event::Instruction::)?;
      }
      satsim::SatEvent::Anomaly(_part_id, _error_string) => {}
    }
    // let state = sat.handle_event(event)?;
  }
  Ok(())
}

#[test]
fn test_should_not_be_able_to_directly_construct_traits() {
  // let mut state = machine::into_iter();
  // println!("state: {:?}",state);
  // while let Ok(state) = machine::next() {
  //   println!("state: {:?}",state);

  // }
}
