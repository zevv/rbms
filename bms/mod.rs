

pub mod dev;
pub mod plat;
pub mod rv;
pub mod evq;

//use core::fmt::{self, Write};

//use crate::bms::plat::Plat;
//use crate::bms::plat::bms::Bms;



pub fn bms() -> Result<(), rv::Rv> {
   
    let evq = evq::Evq::new();
    evq.lock().unwrap().init();

    evq.lock().unwrap().register(evq::EvType::Tick10Hz as u8, |ev| {
        println!("Tick10Hz");
    });

    let mut devmgr = dev::Devmgr::new();

    //let plat = Box::new(plat::bms::linux::Linux{ value: 42 });
    let mut plat = plat::bms::linux::new(&mut devmgr, evq.clone());
    plat.init(&mut devmgr)?;
    devmgr.init()?;

    plat.devs().uart.uart0.borrow().write(b"=== Hello, world! ===\n")?;
    plat.devs().gpio.backlight.borrow_mut().set(true)?;
    plat.devs().gpio.charge.borrow_mut().get()?;

    devmgr.dump()?;

    evq.lock().unwrap().push(evq::Event::Boot);
    evq.lock().unwrap().run();

    Ok(())

    // print typeof of plat
}

