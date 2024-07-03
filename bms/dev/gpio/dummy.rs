

use std::fmt;
use std::sync::{Mutex};
use super::super::Dev;
use super::Gpio;
use super::super::Kind;
use crate::bms::evq::Evq;
use crate::bms::rv::Rv;

struct Dummy {
    name: &'static str,
    pin: u8,
    state: Mutex<bool>,
}

pub fn new(name: &'static str, _: &Evq, pin: u8) -> &'static (dyn Gpio + Sync) {
    return Box::leak(Box::new(Dummy {
        name: name,
        pin: pin,
        state: Mutex::new(false),
    }));
}


impl Dev for Dummy {
    fn init(&'static self) -> Rv {
        Rv::Ok
    }

    fn kind(&self) -> Kind {
        return Kind::Gpio;
    }

    fn get_name(&self) -> &str {
        self.name
    }
    
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "dummy@{}", self.pin)
    }

    fn as_dev(&self) -> &(dyn Dev + Sync) {
        self
    }

    fn as_gpio(&self) -> Option<&dyn Gpio> {
        Some(self)
    }
}


impl Gpio for Dummy {
    fn set(&self, state: bool) -> Rv {
        *self.state.lock().unwrap() = state;
        println!("Set pin {} to {}", self.pin, state);
        Rv::Ok
    }

    fn get(&self) -> bool {
        *self.state.lock().unwrap()
    }
    
}

