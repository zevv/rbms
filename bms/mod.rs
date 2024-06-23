

pub mod evq;
pub mod rv;
pub mod dev;
pub mod plat;

use evq::Event;


pub fn bms() {

    let evq = evq::Evq::new();
    let mut devmgr = dev::Mgr::new();

    let plat: &'static dyn plat::bms::Bms = plat::bms::linux::new(evq, &mut devmgr);
    plat.init();

    devmgr.init();
    devmgr.dump();

    plat.devs().uart.uart0.write("=== Hello\n".as_bytes());
    plat.devs().gpio.backlight.set(false);

    evq.reg(|e| {
        match e {
            Event::Tick1Hz => {
                let stats = plat.devs().uart.uart0.get_stats();
                println!("uart: {:?}", stats);
            }
            _ => {
                println!("Other event");
            }
        }
    });


    evq.run();
}

