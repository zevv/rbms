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

pub fn new(evq: &'static evq::Evq, devmgr: &'static dev::Mgr, climgr: &'static cli::Mgr) -> &'static dyn Plat {

    let uart0 = dev::uart::linux::new(evq, "/dev/stdout");

    let cli = climgr.add_cli(|c| {
        let buf = [c as u8];
        uart0.write(&buf);
    });

    let plat = Box::leak(Box::new(Linux {
        evq: evq,
        devs: plat::Devices {
            gpio: plat::Gpio {
                backlight: dev::gpio::dummy::new(evq, 13),
                charge: dev::gpio::dummy::new(evq, 28),
                discharge: dev::gpio::dummy::new(evq, 5),
            },
            uart: plat::Uart { uart0: uart0 },
        },
        climgr: climgr,
    }));

    devmgr.add(plat.devs.gpio.backlight);
    devmgr.add(plat.devs.gpio.charge);
    devmgr.add(plat.devs.gpio.discharge);
    devmgr.add(plat.devs.uart.uart0);

    evq.reg("plat", |e| match e {
        Event::Uart { dev, data, len } => {
            if dev.eq(uart0) {
                for i in 0..(*len as usize) {
                    cli.handle_char(data[i]);
                }
            }
        }
        _ => {}
    });

    plat
}
