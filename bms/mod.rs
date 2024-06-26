#[macro_use]
pub mod log;
pub mod cli;
pub mod dev;
pub mod evq;
pub mod plat;
pub mod rv;



pub fn bms() {
    let climgr = cli::Mgr::new();
    let evq = evq::Evq::new(climgr);
    let devmgr = dev::Mgr::new(climgr);

    #[cfg(feature = "linux")]
    let plat: &'static dyn plat::Plat = plat::linux::new(evq, devmgr, climgr);

    #[cfg(feature = "nowos")]
    let plat: &'static dyn plat::Plat = plat::nowos::new(evq, devmgr, climgr);

    log::set_console(plat.console());

    plat.climgr().reg("quit", "bye bye", |_cli, _args| {
        evq.stop();
        rv::Rv::Ok
    });

    plat.init();
    devmgr.init();
    
    let _console = plat.devs().uart.uart0;

    plat.devs().gpio.backlight.set(true);

    //evq.reg_filter(evq::EvType::Uart, |ev| {
    //    linf!("filter 2");
    //});

    log::inf!("=== start ===");

    evq.run();
}
