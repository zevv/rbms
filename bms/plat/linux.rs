use crate::bms::cli;
use crate::bms::dev;
use crate::bms::evq;
use crate::bms::evq::Event;
use crate::bms::plat;
use crate::bms::plat::Plat;
use crate::bms::rv::Rv;
use std::thread;

pub struct Linux {
    evq: &'static evq::Evq,
    devs: plat::Devices,
    climgr: &'static cli::Mgr,
}

impl Plat for Linux {
    fn init(&self) -> Rv {
        let sender = self.evq.sender();
        thread::spawn(move || loop {
            thread::sleep(std::time::Duration::from_millis(1000));
            sender.send(evq::Event::Tick1Hz {}).unwrap();
        });
        return Rv::Ok;
    }

    fn devs(&self) -> &plat::Devices {
        &self.devs
    }

    fn climgr(&self) -> &cli::Mgr {
        &self.climgr
    }

    fn console(&self) -> &'static (dyn dev::uart::Uart + Sync) {
        self.devs.uart.uart0
    }
}

pub fn new(
    evq: &'static evq::Evq,
    devmgr: &'static dev::Mgr,
    climgr: &'static cli::Mgr,
) -> &'static dyn Plat {

    let plat = Box::leak(Box::new(Linux {
        evq: evq,
        devs: plat::Devices {
            gpio: plat::Gpio {
                backlight: devmgr.add(dev::gpio::dummy::new("backlight", evq, 13)),
                charge: devmgr.add(dev::gpio::dummy::new("charge", evq, 28)),
                discharge: devmgr.add(dev::gpio::dummy::new("discharge", evq, 5)),
            },
            uart: plat::Uart {
                uart0: devmgr.add(dev::uart::linux::Linux::new("uart0", evq, "/dev/stdout")),
            },
        },
        climgr: climgr,
    }));

    let cli = climgr.add_cli(|c| {
        let buf = [c as u8];
        plat.devs.uart.uart0.write(&buf);
    });

    evq.reg("plat", |e| match e {
        Event::Uart { dev, data, len } => {
            if dev.eq(plat.devs.uart.uart0) {
                for i in 0..(*len as usize) {
                    cli.handle_char(data[i]);
                }
            }
        }
        _ => {}
    });

    plat
}
