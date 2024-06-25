#[macro_use]
pub mod log;
pub mod cli;
pub mod dev;
pub mod evq;
pub mod plat;
pub mod rv;



pub fn bms() {
    let evq = evq::Evq::new();
    let devmgr = dev::Mgr::new();

    #[cfg(feature = "linux")]
    let plat: &'static dyn plat::Plat = plat::linux::new(evq, devmgr);

    #[cfg(feature = "nowos")]
    let plat: &'static dyn plat::Plat = plat::nowos::new(evq, devmgr);

    log::set_console(plat.console());

    plat.climgr().reg("quit", "bye bye", |_cli, _args| {
        evq.stop();
        rv::Rv::Ok
    });

    plat.init();
    devmgr.init();
    devmgr.dump();
    
    let console = plat.devs().uart.uart0;

    plat.devs().gpio.backlight.set(true);

    linf!("=== start ===");

    evq.run();
}
