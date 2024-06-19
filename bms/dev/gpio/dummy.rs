

use super::super::Dev;
use super::Gpio;
use crate::bms::rv::*;

pub struct Dummy {
    pin: u32,
}

pub fn new(pin: u32) -> Box::<dyn Gpio> {
    Box::new(Dummy {
        pin: pin,
    })
}

impl Dev for Dummy {
    fn init(&mut self) -> Result<(), Rv> {
        Err(Rv::ErrImpl)
    }
}

impl Gpio for Dummy {
    fn set(&self, val: bool) -> Result<(), Rv> {
        println!("Dummy::set({}) : {}", self.pin, val);
        Ok(())
    }
}


