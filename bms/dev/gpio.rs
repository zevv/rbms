
pub mod dummy;

use crate::bms::cli;
use crate::bms::dev;

#[cfg(feature = "esp32")]
pub mod esp32;

use super::Dev;
use crate::bms::rv::Rv;

pub trait Gpio : Dev {
    fn set(&self, state: bool) -> Rv;
    fn get(&self) -> bool;
}


pub struct Mgr {
    climgr: &'static cli::Mgr,
}


impl Mgr {

    pub fn new(climgr: &'static cli::Mgr, devmgr: &'static dev::Mgr) -> &'static Mgr {

        
        let mgr = Box::leak(Box::new(Mgr {
            climgr: climgr,
        }));

        climgr.reg("gpio", "", |cli, args| {
        
            let mut rv = Rv::ErrInval;

            match args {
                [ "list" ] => {
                    devmgr.foreach_dev(|dev| {
                        if let Some(gpio) = dev.as_any().downcast_ref::<&dyn Gpio>() {
                            cli.printf(format_args!("{:?} {}\n", dev, gpio.get()));
                        }
                    });
                    rv = Rv::Ok
                },
                _ => {
                    cli.printf(format_args!("unknown command\n"));
                }
            }

            rv
        });

        mgr
    }


}

