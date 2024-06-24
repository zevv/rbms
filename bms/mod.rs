

pub mod evq;
pub mod rv;
pub mod dev;
pub mod plat;
pub mod cli;

use evq::Event;



pub fn bms() {

    let evq = evq::Evq::new();
    let mut devmgr = dev::Mgr::new();

    #[cfg(feature = "linux")]
    let plat: &'static dyn plat::bms::Bms = plat::bms::linux::new(evq, &mut devmgr);
    
    #[cfg(feature = "nowos")]
    let plat: &'static dyn plat::bms::Bms = plat::bms::nowos::new(evq, &mut devmgr);

    let cli = cli::new(|c| {
        let buf = [ c as u8 ];
        plat.devs().uart.uart0.write(&buf);
    });

    cli.reg("help", |args| {
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
            Event::Tick1Hz => {
                //println!("event: Tick1Hz");
                //println!("{:?}", plat.devs().uart.uart0.get_stats());
            }
            Event::Uart { dev, data, len } => {
                if console.eq(*dev) {
                    for i in 0..(*len as usize) {
                        cli.handle_char(data[i] as char);
                    }
                }
            }
        }
    });


    evq.run();
}

