
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

use std::thread;
use crate::bms::plat::Plat;
use crate::bms::plat;
use crate::bms::plat::Bms;
use crate::bms::evq;
use crate::bms::dev;
use crate::bms::rv::Rv;


pub struct Nowos {
    devs: bms::Devices,
    evq: &'static evq::Evq,
}


impl Plat for Nowos {
    fn init(&self) -> Rv {
        let sender = self.evq.sender();
        thread::spawn(move || {
            loop {
                thread::sleep(std::time::Duration::from_millis(1000));
                sender.send(evq::Event::Tick1Hz {}).unwrap();
            }
        });
        return Rv::Ok;
    }

    fn devs(&self) -> &bms::Devices {
        &self.devs
    }
}


pub fn new(evq: &'static evq::Evq, devmgr: &mut dev::Mgr) -> &'static dyn Plat {

    let peripherals = Peripherals::take().unwrap();
    let pin = peripherals.pins.gpio15.downgrade();
    //let mut led = PinDriver::output(peripherals.pins.gpio15).unwrap();
    //led.set_high().unwrap();

    let plat = Box::leak(Box::new(Nowos {
        evq: evq,
        devs: bms::Devices {
            gpio: bms::Gpio {
                backlight: dev::gpio::esp32::new(evq, pin),
                charge: dev::gpio::dummy::new(evq, 28),
                discharge: dev::gpio::dummy::new(evq, 5),
            },

            uart: bms::Uart {
                uart0: dev::uart::esp32::new(evq),
            },
        },
    }));

    devmgr.add(plat.devs.gpio.backlight);
    devmgr.add(plat.devs.gpio.charge);
    devmgr.add(plat.devs.gpio.discharge);
    devmgr.add(plat.devs.uart.uart0);

    return plat
}


