
pub mod linux;

use std::rc::Rc;
use std::cell::RefCell;
use super::Plat;
use crate::bms::dev;

pub struct Gpio {
    pub backlight: Rc<RefCell<dyn dev::gpio::Gpio>>,
    pub charge: Rc<RefCell<dyn dev::gpio::Gpio>>,
    pub discharge: Rc<RefCell<dyn dev::gpio::Gpio>>,
}

pub struct Uart {
    pub uart0: Rc<RefCell<dyn dev::uart::Uart>>,
}
    
pub struct Devices {
    pub gpio: Gpio,
    pub uart: Uart,

}


pub trait Bms : Plat {
    fn devs(&self) -> &Devices;

}


