
pub mod linux;

use super::Plat;
use crate::bms::dev;

pub struct Gpio {
    pub backlight: Box::<dyn dev::gpio::Gpio>,
    pub charge: Box::<dyn dev::gpio::Gpio>,
    pub discharge: Box::<dyn dev::gpio::Gpio>,
}

pub struct Uart {
    pub uart0: Box::<dyn dev::uart::Uart>,
}
    
pub struct Devices {
    pub gpio: Gpio,
    pub uart: Uart,

}


pub trait Bms : Plat {
    fn devs(&self) -> &Devices;

}


