#![allow(dead_code)]

extern crate nom;

use std::io::Error;
mod satellite;
mod satsim;
mod state_machine;

fn main() -> Result<(), Error> {
  // state_machine::main()?;
  // satellite::main()?;
  satsim::main()?;
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
