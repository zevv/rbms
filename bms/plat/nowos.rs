
use esp_idf_hal::peripherals::Peripherals;

use std::thread;
use crate::bms::cli;
use crate::bms::log;
use crate::bms::plat::Plat;
use crate::bms::plat;
use crate::bms::evq;
use crate::bms::dev;
use crate::bms::rv::Rv;

pub struct Nowos {
    devs: plat::Devices,
    evq: &'static evq::Evq,
    climgr: &'static cli::Mgr,
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

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let uart0 = dev::uart::esp32::new(
        evq, 
        peripherals.uart1,
        pins.gpio1, pins.gpio3);

    let cli = climgr.add_cli(|c| {
        let buf = [c as u8];
        uart0.write(&buf);
    });

    let plat = Box::leak(Box::new(Nowos {
        evq: evq,
        climgr: cli::Mgr::new(),
        devs: plat::Devices {
            gpio: plat::Gpio {
                backlight: dev::gpio::esp32::new(evq, pins.gpio15),
                charge: dev::gpio::dummy::new(evq, 28),
                discharge: dev::gpio::dummy::new(evq, 5),
            },

            uart: plat::Uart {
                uart0: uart0,
            },
        },
    }));

    devmgr.add(plat.devs.gpio.backlight);
    devmgr.add(plat.devs.gpio.charge);
    devmgr.add(plat.devs.gpio.discharge);
    devmgr.add(plat.devs.uart.uart0);


    evq.reg("plat", |e| {
        match e {
            evq::Event::Uart { dev, data, len } => {
                if dev.eq(uart0) {
                    for i in 0..(*len as usize) {
                        cli.handle_char(data[i]);
                    }
                }
            }
            _ => {}
       }});

    return plat
}


