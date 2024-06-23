

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

    plat.devs().uart.uart0.write(b"=== Hello ===\n");
    plat.devs().gpio.backlight.set(false);
    
    evq.reg(|e| {
        match e {
            Event::Tick1Hz => {
                println!("event: Tick1Hz");
                println!("{:?}", plat.devs().uart.uart0.get_stats());
            }
            Event::Uart { dev, data, len } => {
                println!("event: Uart dev={:?} len={:?}, data={:?}", *dev as &(dyn dev::Dev + Sync), len, data);
            }
        }
    });


    evq.run();
}

