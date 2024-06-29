


use std::fmt;
use std::sync::{Mutex};

use esp_idf_svc::hal::gpio::*;

use super::super::Dev;
use super::Gpio;
use super::super::Kind;
use crate::bms::evq::Evq;
use crate::bms::rv::Rv;


struct Dd<T: Pin> {
    pin: PinDriver<'static, T, Output>,
}

struct Esp32<T: Pin> {
    dd: Mutex<Dd<T>>,
}

pub fn new<T: OutputPin>(_: &Evq, pin: T) -> &'static (dyn Gpio + Sync) {
    let gpio = Box::leak(Box::new(Esp32::<T> {
        dd: Mutex::new(Dd::<T> {
            pin: PinDriver::output(pin).unwrap(),
        }),
    }));
    gpio
}


impl<T: Pin> Dev for Esp32<T> {
    fn init(&'static self) -> Rv {
        Rv::Ok
    }

    fn kind(&self) -> Kind {
        return Kind::Gpio;
    }
    
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _dd = self.dd.lock().unwrap();
        return write!(f, "esp32@");
    }
}


impl<T: Pin> Gpio for Esp32<T> {
    fn set(&self, state: bool) -> Rv {
        let mut dd = self.dd.lock().unwrap();
        if state {
            dd.pin.set_high().unwrap();
        } else {
            dd.pin.set_low().unwrap();
        }
        Rv::Ok
    }
}

