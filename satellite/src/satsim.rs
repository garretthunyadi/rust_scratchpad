use std::io::Error;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

mod batsim;
use batsim::*;

pub fn main() -> Result<(), Error> {
    // Create a thread per subsystem, (includin the main system? core/sat/?)
    // batsim::main()?;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut satsim = SatSim::new(tx);
        satsim.run();
    });

    for received in rx {
        println!("SatSim Rec'd: {:?}", received);
    }

    Ok(())
}

// trait Sim<T,E> {

//     fn run() {

//     }
// }

#[derive(Clone, Debug, PartialEq)]
pub struct Percent(u8);
impl Percent {
    fn inc(&mut self) {
        if self.0 < 100 {
            self.0 += 1;
        }
    }
    fn dec(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }
}

type PartId = String;

#[derive(Clone, Debug, PartialEq)]
enum SatEvent {
    BatteryStateChange(PartId, Percent),
    Anomaly(PartId, String),
}

struct SatSim {
    bat_1_part_id: String,
    // batsim: batsim::BatSim, // how will I communicate to or kill this?
    bat_1_rx: std::sync::mpsc::Receiver<batsim::BatEvent>,
    prev_battery_1_power_level: Percent,

    bat_2_part_id: String,
    bat_2_rx: std::sync::mpsc::Receiver<batsim::BatEvent>,
    prev_battery_2_power_level: Percent,

    comm_tx: std::sync::mpsc::Sender<SatEvent>,
    // _rng: ThreadRng,
    // _d100: rand::distributions::uniform::Uniform<i32>,
    _private: (),
}
impl SatSim {
    fn new(comm_tx: std::sync::mpsc::Sender<SatEvent>) -> SatSim {
        let (tx, bat_1_rx) = mpsc::channel();
        thread::spawn(move || {
            let mut batsim = BatSim::new(tx);
            batsim.run();
        });
        let (tx, bat_2_rx) = mpsc::channel();
        thread::spawn(move || {
            let mut batsim = BatSim::new(tx);
            batsim.run();
        });
        SatSim {
            bat_1_part_id: String::from("battery_1"),
            bat_1_rx: bat_1_rx,
            prev_battery_1_power_level: Percent(0),

            bat_2_part_id: String::from("battery_2"),
            bat_2_rx: bat_2_rx,
            prev_battery_2_power_level: Percent(0),
            comm_tx,
            // _rng: rand::thread_rng(),
            // _d100: Uniform::from(1..100),
            _private: (),
        }
    }
    fn run(&mut self) {
        // for power_level in &self.bat_1_rx {
        //     println!("SatSim Rec'd from BatSim: {:?}", power_level);
        // }
        loop {
            thread::sleep(Duration::from_secs(1));
            if let Ok(e) = &self.bat_1_rx.try_recv() {
                match e {
                    BatEvent::PowerReport(perc) => {
                        if self.prev_battery_1_power_level != *perc {
                            self.comm_tx
                                .send(SatEvent::BatteryStateChange(
                                    self.bat_1_part_id.clone(),
                                    perc.clone(),
                                ))
                                .unwrap();
                            self.prev_battery_1_power_level = perc.clone();
                        }
                    }
                    BatEvent::Anomaly(msg) => {
                        self.comm_tx
                            .send(SatEvent::Anomaly(self.bat_1_part_id.clone(), msg.clone()))
                            .unwrap();
                    }
                }
            }

            if let Ok(e) = &self.bat_2_rx.try_recv() {
                match e {
                    BatEvent::PowerReport(perc) => {
                        if self.prev_battery_2_power_level != *perc {
                            self.comm_tx
                                .send(SatEvent::BatteryStateChange(
                                    self.bat_2_part_id.clone(),
                                    perc.clone(),
                                ))
                                .unwrap();
                            self.prev_battery_2_power_level = perc.clone();
                        }
                    }
                    BatEvent::Anomaly(msg) => {
                        self.comm_tx
                            .send(SatEvent::Anomaly(self.bat_2_part_id.clone(), msg.clone()))
                            .unwrap();
                    }
                }
            }

            // self.systems_check();
            // self.comm
            //     .send(BatEvent::PowerReport(self.power.clone()))
            //     .unwrap();
        }
    }
}

//
//
//

//
//
//

// ==========================================================================
// ==========================================================================
// ==========================================================================
// ==========================================================================
pub fn main1() -> Result<(), Error> {
    // Create a thread per subsystem, (includin the main system? core/sat/?)
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    Ok(())
}
