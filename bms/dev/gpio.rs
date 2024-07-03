pub mod dummy;

use crate::bms::cli;
use crate::bms::dev;
use crate::bms::log;

#[cfg(feature = "esp32")]
pub mod esp32;

use super::Dev;
use crate::bms::rv::Rv;

pub trait Gpio: Dev {
    fn set(&self, state: bool) -> Rv;
    fn get(&self) -> bool;
}

pub struct Mgr {
    climgr: &'static cli::Mgr,
}

impl Mgr {
    pub fn new(climgr: &'static cli::Mgr, devmgr: &'static dev::Mgr) -> &'static Mgr {
        let mgr = Box::leak(Box::new(Mgr { climgr: climgr }));

        climgr.reg(
            "gpio",
            "l[ist] | s[et] <name> <0|1>",
            |cli, args| match args {
                ["list" | "l"] => {
                    devmgr.foreach_dev(|dev| {
                        if let Some(gpio) = dev.as_gpio() {
                            cli.printf(format_args!("{} {}\n", dev, gpio.get()));
                        }
                    });
                    Rv::Ok
                }
                ["set" | "s", name, state] => {
                    if let Some(dev) = devmgr.find_by_name(name) {
                        log::tst!("found dev");
                    }
                    Rv::Ok
                }
                _ => {
                    cli.printf(format_args!("unknown command\n"));
                    Rv::ErrInval
                }
            },
        );

        mgr
    }
}
