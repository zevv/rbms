
use std::thread;
use crate::bms::plat::Plat;
use crate::bms::plat::bms;
use crate::bms::plat::bms::Bms;
use crate::bms::evq;
use crate::bms::dev;
use crate::bms::rv::Rv;


pub struct Linux {
    devs: bms::Devices,
    evq: &'static evq::Evq,
}


impl Plat for Linux {
    fn init(&self) -> Rv {
        let sender = self.evq.sender();
        thread::spawn(move || {
            loop {
                thread::sleep(std::time::Duration::from_millis(1000));
                sender.send(evq::Event::Tick1Hz {});
            }
        });
        return Rv::Ok;
    }
}


impl Bms for Linux {
    fn devs(&self) -> &bms::Devices {
        &self.devs
    }
}


pub fn new(evq: &'static evq::Evq, devmgr: &mut dev::Mgr) -> &'static dyn Bms {

    let plat = Box::leak(Box::new(Linux {
        evq: evq,
        devs: bms::Devices {
            gpio: bms::Gpio {
                backlight: dev::gpio::dummy::new(evq, 13),
                charge: dev::gpio::dummy::new(evq, 28),
                discharge: dev::gpio::dummy::new(evq, 5),
            },

            uart: bms::Uart {
                uart0: dev::uart::linux::new(evq, "/dev/stdout"),
            },
        },
    }));

    devmgr.add(plat.devs.gpio.backlight);
    devmgr.add(plat.devs.gpio.charge);
    devmgr.add(plat.devs.gpio.discharge);
    devmgr.add(plat.devs.uart.uart0);

    return plat
}


