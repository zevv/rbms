

use std::rc::Rc;
use std::cell::RefCell;

use crate::bms::rv::Rv;
use crate::bms::dev;
use crate::bms::dev::Dev;
use crate::bms::dev::gpio::Gpio;

pub struct Dummy {
    pin: u32,
}

pub fn new(pin: u32) -> Rc<RefCell<dyn Gpio>> {
    Rc::new(RefCell::new(Dummy {
        pin: pin,
    }))
}

impl Dev for Dummy {

    fn kind(&self) -> dev::Kind {
        dev::Kind::Gpio
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

    fn get(&self) -> Result<bool, Rv> {
        println!("dev::gpio::Dummy.get({})", self.pin);
        Ok(false)
    }
}


