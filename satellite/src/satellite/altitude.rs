#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Ready,
    NotReady,
}

struct Altitude(u32);
pub struct ControlUnit {
    _private: (), // prevent construction
    current: Option<Altitude>,
}
impl ControlUnit {
    pub fn new() -> ControlUnit {
        ControlUnit {
            _private: (), // prevent construction
            current: None,
        }
    }
    pub fn calibrate(&mut self) -> Result<(), std::io::Error> {
        self.current = Some(Altitude(100));
        Ok(())
    }
}
