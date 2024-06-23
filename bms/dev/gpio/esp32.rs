


use std::fmt;
use std::sync::{Mutex};

use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::hal::gpio::*;
use esp_idf_hal::sys::esp;
use esp_idf_sys::*;

use super::super::Dev;
use super::Gpio;
use super::super::Kind;
use crate::bms::evq::Evq;
use crate::bms::rv::Rv;


struct Dd {
    pin: AnyIOPin,
}

struct Esp32 {
    dd: Mutex<Dd>,
}

pub fn new(_: &Evq, pin: AnyIOPin) -> &'static (dyn Gpio + Sync) {
    return Box::leak(Box::new(Esp32 {
        dd: Mutex::new(Dd {
            pin: pin,
        }),
    }));
    esp!(unsafe { gpio_set_direction(pin.pin(), gpio_mode_t_GPIO_MODE_OUTPUT) });
}


impl Dev for Esp32 {
    fn init(&'static self) -> Rv {
        Rv::Ok
    }

    fn kind(&self) -> Kind {
        return Kind::Gpio;
    }
    
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dd = self.dd.lock().unwrap();
        return write!(f, "esp32@{}", dd.pin.pin());
    }
}


impl Gpio for Esp32 {
    fn set(&self, state: bool) -> Rv {
        let dd = self.dd.lock().unwrap();
        esp!(unsafe { gpio_set_level(dd.pin.pin(), state as u32) });
        println!("esp32: set {} to {}", dd.pin.pin(), state);
        Rv::Ok
    }
}

