
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

pub struct Base {
    evq: &'static crate::bms::evq::Evq,
    climgr: &'static crate::bms::cli::Mgr,
    console: Option<&'static (dyn dev::uart::Uart + Sync)>,
    devs: Devices,
}

pub trait Plat {
    fn init(&self) -> Rv;
    fn base(&self) -> &Base;
    fn devs(&self) -> &Devices { &self.base().devs }
    fn climgr(&self) -> &crate::bms::cli::Mgr { self.base().climgr }
    fn console(&self) -> Option<&'static (dyn dev::uart::Uart + Sync)> { self.base().console }
}


