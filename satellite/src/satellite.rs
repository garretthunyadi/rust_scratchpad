use std::io::{Error, ErrorKind};
mod altitude;
mod attitude;
mod laser;
mod power;

fn error(msg: &str) -> Result<State, Error> {
    Err(Error::new(ErrorKind::Other, msg))
}

pub fn main() -> Result<(), Error> {
    use self::Instruction::*;
    use Event::*;

    let mut s = Satellite::new();
    let good_event_set = vec![
        Instruction(Wake),
        PowerEvent(power::Event::Ready),
        LaserEvent(laser::Event::Ready),
        // AltitudeEvent(altitude::Event::Ready),
        // AttitudeEvent(attitude::Event::Ready),
        Instruction(PointTo(Coord(1, 2, 3))),
        LaserEvent(laser::Event::Ready),
        Instruction(Shoot),
    ];
    let bad_event_set_1 = vec![Instruction(Wake), Instruction(Shoot)];

    s.handle(&good_event_set)?;
    let res = s.handle(&bad_event_set_1);
    println!("expect error: {:?}", res);
    // let res = s.handle(&bad_event_set_2);
    // println!("expect error: {:?}", res);
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Instruction(Instruction),
    PowerEvent(power::Event),
    // AttitudeEvent(attitude::Event),
    // AltitudeEvent(altitude::Event),
    LaserEvent(laser::Event),
}

type InstructionSet = Vec<Instruction>;

#[derive(Clone, Debug, PartialEq)]
pub struct Coord(pub u32, pub u32, pub u32);

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    PointTo(Coord),
    PowerSave,
    Wake,
    Shoot,
}

type Time = u128; // something like unix epoch, but maybe simplify to just monotonicall increasing houts

pub struct Satellite {
    _private: (), // prevent construction
    attitude: attitude::ControlUnit,
    altitude: altitude::ControlUnit,
    laser: laser::ControlUnit,
    power: power::ControlUnit,
}
impl Satellite {
    pub fn new() -> Satellite {
        Satellite {
            _private: (), // prevent construction
            attitude: attitude::ControlUnit::new(),
            altitude: altitude::ControlUnit::new(),
            laser: laser::ControlUnit::new(),
            power: power::ControlUnit::new(),
        }
    }
    pub fn handle(&mut self, events: &Vec<Event>) -> Result<(), std::io::Error> {
        for event in events {
            self.handle_event(event)?;
        }
        Ok(())
    }
    fn handle_event(&mut self, event: &Event) -> Result<State, Error> {
        use self::Instruction::*;
        use Event::*;
        use State::*;

        // Instruction(Instruction),
        // PowerEvent(power::Event),
        // AttitudeEvent(attitude::Event),
        // AltitudeEvent(altitude::Event),
        // LaserEvent(laser::Event),
        // PointTo(Coord),
        // PowerSave,
        // Wake,
        // Shoot,
        match (self.state(), event) {
            // Initializing, Instruction::Wake) => Ok(State::In),
            (_, Instruction(Wake)) => self.wake(),
            (Unknown, _) => error("Unknown state. Not executing"),
            (_, PowerEvent(e)) => self.power.handle_event(e).and_then(|_| Ok(self.state())),
            (_, LaserEvent(e)) => self.laser.handle_event(e).and_then(|_| Ok(self.state())),

            (LowPower, Instruction(PointTo(_))) => panic!(),
            (Sleeping, Instruction(PointTo(_))) => panic!(),
            // (LowPower,
            // Sleeping,
            // ArmedUntargeted,
            // ArmedTargeted,
            (ArmedUntargeted, Instruction(PointTo(_))) => self
                .laser
                .handle_event(&laser::Event::StateChange(laser::State::ArmedTargeted))
                .and_then(|_| Ok(self.state())),
            (ArmedUntargeted, Instruction(Shoot)) => error("Not pointing, can't shoot."),
            // Firing,
            // Cooling,
            // (PointTo(coord)),
            // PowerSave,
            // Wake,

            // this is an invalid state, but it is being handles at runtime. :(
            (Initializing, Instruction(Shoot)) => {
                panic!()
                // error("Initializing, cannot shoot. Not executing")
            }
            (a, b) => {
                println!("({:?},{:?}) => ", a, b);
                Ok(Unknown)
            }
        }
    }
    fn state(&self) -> State {
        use State::*;

        match (self.power.state(), self.laser.state()) {
            (power::State::Unknown, _) => Unknown,
            (_, laser::State::Unknown) => Unknown,

            (power::State::Initializing, _) => Initializing,
            (_, laser::State::Initializing) => Initializing,

            (power::State::LowPower, _) => LowPower,
            (_, laser::State::LowPower) => LowPower,

            (power::State::Sleeping, _) => Sleeping,
            (_, laser::State::Sleeping) => Sleeping,

            (power::State::Hot, laser::State::Firing) => Firing,
            (_, laser::State::Firing) => Firing,

            (power::State::Hot, _) => Cooling,
            (_, laser::State::Cooling) => Cooling,

            (power::State::Normal, laser::State::ArmedUntargeted) => ArmedUntargeted,
            (power::State::Normal, laser::State::ArmedTargeted) => ArmedTargeted,
        }
    }
    fn calibrate(&mut self) -> Result<State, Error> {
        self.attitude.calibrate()?;
        self.altitude.calibrate()?;
        self.laser.calibrate()?;
        self.power.calibrate()?;
        Ok(State::Ready)
    }

    fn wake(&mut self) -> Result<State, Error> {
        self.calibrate()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum State {
    Unknown,
    Initializing,
    LowPower,
    Sleeping,
    ArmedUntargeted,
    ArmedTargeted,
    Firing,
    Cooling,
    Ready,
}

#[test]
fn test_satellite() {
    // use Instruction::*;
    // let mut s = Satellite::new();
    // assert!(s
    //     .handle(&vec![Wake, PowerSave, Wake, PointTo(Coord(1, 2, 3)), Shoot])
    //     .is_ok());
    // assert!(s
    //     .handle(&vec![Wake, PowerSave, Wake, PointTo(Coord(1, 2, 3)), Shoot])
    //     .is_err());
    // assert!(s
    //     .handle(&vec![Wake, PowerSave, Wake, PointTo(Coord(1, 2, 3)), Shoot])
    //     .is_err());
}
