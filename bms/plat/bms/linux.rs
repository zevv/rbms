
use super::super::Plat;
use super::Bms;

use crate::bms::plat::bms;
use crate::bms::dev;
use crate::bms::dev::Devmgr;
use crate::bms::rv::*;

pub struct Linux {
    devs: bms::Devices,

}


pub fn new(devmgr: &mut dev::Devmgr) -> Box::<dyn Bms> {

    let plat = Box::new(Linux {
        devs: bms::Devices {
            gpio: bms::Gpio {
                backlight: dev::gpio::dummy::new(13),
                charge: dev::gpio::dummy::new(28),
                discharge: dev::gpio::dummy::new(5),
            },

            uart: bms::Uart {
                uart0: dev::uart::linux::new("/dev/stdout"),
            },
        }

    });

    // TODO
    //devmgr.add(&*plat.devs.uart.uart0);

    plat
}


impl Plat for Linux {
    fn init(&mut self, _devmgr: &mut Devmgr) -> Result<(), Rv> {

        let res = self.devs.uart.uart0.init();
        match res {
            Ok(_) => {},
            Err(e) => {
                println!("PlatLinux::init() uart0.init() failed: {}", e);
            }
        }

        Ok(())

    }
}


impl Bms for Linux {

    fn devs(&self) -> &bms::Devices {
        &self.devs
    }
}

