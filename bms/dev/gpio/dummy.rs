use super::super::Dev;
use super::super::Kind;
use super::Gpio;
use crate::bms::dev;
use crate::bms::evq::Evq;
use crate::bms::rv::Rv;
use std::fmt;
use std::sync::Mutex;

struct Dummy {
    base: dev::Base,
    pin: u8,
    state: Mutex<bool>,
}

pub fn new(name: &'static str, _: &Evq, pin: u8) -> &'static (dyn Gpio + Sync) {
    return Box::leak(Box::new(Dummy {
        base: dev::Base {
            kind: Kind::Gpio,
            name: name,
        },
        pin: pin,
        state: Mutex::new(false),
    }));
}

impl Dev for Dummy {
    fn init(&'static self) -> Rv {
        Rv::Ok
    }

    fn base(&self) -> &dev::Base {
        &self.base
    }

    fn kind(&self) -> Kind {
        return Kind::Gpio;
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
