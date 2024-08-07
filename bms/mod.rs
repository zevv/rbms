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
    let _gpiomgr = dev::gpio::Mgr::new(climgr, devmgr);

    #[cfg(feature = "linux")]
    let plat: &'static dyn plat::Plat = plat::linux::new(evq, devmgr, climgr);

    #[cfg(feature = "nowos")]
    let plat: &'static dyn plat::Plat = plat::nowos::new(evq, devmgr, climgr);

    if let Some(c) = plat.console() {
        log::set_console(c);
    }

    plat.climgr().reg("quit", "bye bye", |_cli, _args| {
        evq.stop();
        rv::Rv::Ok
    });

    plat.init();
    devmgr.init();

    let uart0 = plat.devs().uart.uart0;
    log::set_console(uart0);
    

    plat.devs().gpio.backlight.set(true);

    evq.reg_filter("test", evq::EvType::Tick1Hz, move |_ev| {
        //log::inf!("tick1hz");
        //plat.devs().gpio.backlight.set(true);
    });


    //evq.reg_filter(evq::EvType::Uart, |ev| {
    //    linf!("filter 2");
    //});


    log::inf!("event loop start");
    evq.run();
    log::inf!("event loop end");
}
