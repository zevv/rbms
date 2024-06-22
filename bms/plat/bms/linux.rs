
use std::thread;
use std::sync::{Arc, Mutex};


use crate::bms::dev::Devmgr;
use crate::bms::dev;
use crate::bms::evq::*;
use crate::bms::plat::Plat;
use crate::bms::plat::bms::Bms;
use crate::bms::plat::bms;
use crate::bms::rv::*;


pub struct Linux {
    devs: bms::Devices,
    evq: Arc<Mutex<Evq>>,

}


pub fn new(devmgr: &mut dev::Devmgr, evq: Arc<Mutex<Evq>>) -> Box::<dyn Bms> {

    let plat = Box::new(Linux {
        devs: bms::Devices {
            gpio: bms::Gpio {
                backlight: dev::gpio::dummy::new(13),
                charge: dev::gpio::dummy::new(28),
                discharge: dev::gpio::dummy::new(5),
            },

            uart: bms::Uart {
                uart0: dev::uart::linux::new(evq.lock().unwrap().sender(), "/dev/stdout"),
            },
        },
        evq: evq.clone(),

    });

    devmgr.add(plat.devs.uart.uart0.clone());
    devmgr.add(plat.devs.gpio.backlight.clone());
    devmgr.add(plat.devs.gpio.charge.clone());
    devmgr.add(plat.devs.gpio.discharge.clone());

    plat
}


impl Linux {
    fn timer_thread(&self) {
        loop {
            thread::sleep(std::time::Duration::from_millis(1000));
            self.evq.lock().unwrap().push(Event::Tick1Hz {});
        }
    }
}


impl Plat for Linux {

    fn init(&mut self, _devmgr: &mut Devmgr) -> Result<(), Rv> {

        self.evq.lock().unwrap().push(Event::Tick10Hz {});

        // TODO wtf

        // thread::spawn(move || {
        //     loop {
        //         //sender.send(Event::Tick1Hz {});
        //         //evq.lock().unwrap().push(Event::Tick1Hz {});
        //         println!("Linux::init() thread");
        //         thread::sleep(std::time::Duration::from_millis(1000));
        //     }
        // });

        Ok(())

    }

}


impl Bms for Linux {

    fn devs(&self) -> &bms::Devices {
        &self.devs
    }
}

