
#[cfg(feature = "linux")]
pub mod linux;

#[cfg(feature = "nowos")]
pub mod nowos;

use super::rv::*;
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

pub trait Plat {
    fn init(&self) -> Rv;
    fn devs(&self) -> &Devices;
    fn climgr(&self) -> &crate::bms::cli::Mgr;
    fn console(&self) -> &'static (dyn dev::uart::Uart + Sync);
}


