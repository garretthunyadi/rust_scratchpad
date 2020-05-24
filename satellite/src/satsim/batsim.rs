use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;

use std::io::Error;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use super::Percent;

pub fn main() -> Result<(), Error> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut batsim = BatSim::new(tx);
        batsim.run();
    });

    for received in rx {
        println!("Got: {:?}", received);
    }

    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
pub enum BatEvent {
    PowerReport(Percent),
    Anomaly(String),
}
pub struct BatSim {
    power: Percent,
    comm: std::sync::mpsc::Sender<BatEvent>,
    _rng: ThreadRng,
    _d100: rand::distributions::uniform::Uniform<i32>,
    _private: (),
}
impl BatSim {
    pub fn new(comm: std::sync::mpsc::Sender<BatEvent>) -> BatSim {
        BatSim {
            power: Percent(0),
            comm,
            _rng: rand::thread_rng(),
            _d100: Uniform::from(1..100),
            _private: (),
        }
    }
    pub fn run(&mut self) {
        loop {
            thread::sleep(Duration::from_secs(1));
            self.power_check();
            self.comm
                .send(BatEvent::PowerReport(self.power.clone()))
                .unwrap();
        }
    }
    fn power_check(&mut self) {
        let roll = self._d100.sample(&mut self._rng);
        if roll < 10 {
            self.power.dec();
        } else if roll > 80 {
            self.power.inc();
        }
    }
}
