pub mod cli;
pub mod dev;
pub mod evq;
pub mod plat;
pub mod rv;

use evq::Event;

pub fn bms() {
    let evq = evq::Evq::new();
    let devmgr = dev::Mgr::new();

    #[cfg(feature = "linux")]
    let plat: &'static dyn plat::bms::Bms = plat::bms::linux::new(evq, devmgr);

    #[cfg(feature = "nowos")]
    let plat: &'static dyn plat::bms::Bms = plat::bms::nowos::new(evq, devmgr);

    plat.climgr().reg("help", |args| {
        println!("help: {:?}", args);
    });

    plat.init();

    devmgr.init();
    devmgr.dump();

    let console = plat.devs().uart.uart0;

    plat.devs().gpio.backlight.set(true);

    console.write(b"=== Hello ===\n");

    evq.reg(|e| {
        match e {
            _ => {}
        }
    });

    evq.run();
}
