
pub mod linux;

use super::Plat;
use crate::bms::dev;

pub struct Gpio {
    pub backlight: &'static (dyn dev::gpio::Gpio + Sync),
    pub charge: &'static (dyn dev::gpio::Gpio + Sync),
    pub discharge: &'static (dyn dev::gpio::Gpio + Sync),
}

pub struct Uart {
    pub uart0: &'static (dyn dev::uart::Uart + Sync),
}
    
pub struct Devices {
    pub gpio: Gpio,
    pub uart: Uart,

}


pub trait Bms : Plat {
    fn devs(&self) -> &Devices;

}


