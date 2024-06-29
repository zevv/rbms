
pub mod dummy;

use crate::bms::cli;
use crate::bms::dev;

#[cfg(feature = "esp32")]
pub mod esp32;

use super::Dev;
use crate::bms::rv::Rv;

pub trait Gpio : Dev {
    fn set(&self, state: bool) -> Rv;
}


pub struct Mgr {
    climgr: &'static cli::Mgr,
}


impl Mgr {

    pub fn new(climgr: &'static cli::Mgr, devmgr: &'static dev::Mgr) -> &'static Mgr {
        let mgr = Box::leak(Box::new(Mgr {
            climgr: climgr,
        }));

        climgr.reg("gpio", "", |cli, _args| {

            devmgr.foreach_dev(|dev| {
                if dev.kind() == dev::Kind::Gpio {
                    cli.printf(format_args!("{:?}: {:?}\n", dev, di.status));
                }
            });
            Rv::Ok
        });

        mgr
    }


}

