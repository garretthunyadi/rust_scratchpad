use std::io::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Ready,
    NotReady,
}

#[derive(Clone, Debug, PartialEq)]
pub enum State {
    Unknown,
    Initializing,
    LowPower,
    Sleeping,
    Normal,
    Hot,
}

pub type Level = u32; // 0..100

struct Power(Level, Level); // two units

pub struct ControlUnit {
    _private: (), // prevent construction
    current: Option<Level>,
    state: State,
}
impl ControlUnit {
    pub fn new() -> ControlUnit {
        ControlUnit {
            _private: (), // prevent construction
            current: None,
            state: State::Unknown,
        }
    }
    pub fn state(&self) -> State {
        self.state.clone()
    }
    pub fn handle_event(&mut self, event: &self::Event) -> Result<State, Error> {
        match (self.state(), event) {
            (_, Event::Ready) => self.state = State::Normal,
            (_, Event::NotReady) => self.state = State::Unknown,
        }
        Ok(self.state())
    }

    pub fn calibrate(&mut self) -> Result<(), Error> {
        self.state = State::Initializing;
        // do complicated stuff...
        self.state = State::Normal;
        Ok(())
    }
}
