use std::io::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Ready,
    NotReady,
}

struct Quaternion(u32, u32, u32, u32);
pub struct ControlUnit {
    _private: (), // prevent construction
    current: Option<Quaternion>,
}
impl ControlUnit {
    pub fn new() -> ControlUnit {
        ControlUnit {
            _private: (), // prevent construction
            current: None,
        }
    }
    pub fn calibrate(&mut self) -> Result<(), Error> {
        self.current = Some(Quaternion(1, 2, 3, 4));
        Ok(())
    }
}
