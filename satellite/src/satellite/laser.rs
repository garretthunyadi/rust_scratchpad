use std::io::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Ready,
    NotReady,
    StateChange(State),
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
}
pub struct ControlUnit {
    state: State,
    _private: (), // prevent construction
}
impl ControlUnit {
    pub fn new() -> ControlUnit {
        ControlUnit {
            state: State::Unknown,
            _private: (), // prevent construction
        }
    }
    pub fn state(&self) -> State {
        self.state.clone()
    }
    pub fn handle_event(&mut self, event: &self::Event) -> Result<State, Error> {
        match (self.state(), event) {
            (_, Event::Ready) => self.state = State::ArmedUntargeted,
            (_, Event::NotReady) => self.state = State::Unknown,
            (_, Event::StateChange(s)) => self.state = s.clone(),
        }
        Ok(self.state())
    }
    pub fn calibrate(&mut self) -> Result<(), std::io::Error> {
        self.state = State::Initializing;
        // do complicated stuff...
        self.state = State::ArmedUntargeted;
        Ok(())
    }
}
