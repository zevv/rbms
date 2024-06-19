

use std::rc::Rc;
use std::cell::RefCell;
use super::super::Dev;
use super::Gpio;
use crate::bms::rv::*;

pub struct Dummy {
    pin: u32,
}

pub fn new(pin: u32) -> Rc<RefCell<dyn Gpio>> {
    Rc::new(RefCell::new(Dummy {
        pin: pin,
    }))
}

impl Dev for Dummy {

    // TODO: super?
    fn kind(&self) -> super::super::Kind {
        super::super::Kind::Gpio
    }

    fn init(&mut self) -> Result<(), Rv> {
        println!("dev::gpio::Dummy.init({})", self.pin);
        Err(Rv::ErrImpl)
    }
}

impl Gpio for Dummy {
    fn set(&self, val: bool) -> Result<(), Rv> {
        println!("dev::gpio::Dummy.set({}) : {}", self.pin, val);
        Ok(())
    }
}

