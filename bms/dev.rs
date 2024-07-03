pub mod gpio;
pub mod uart;

use crate::bms::cli;
use crate::bms::rv::Rv;
use crate::bms::log;
use std::cell::RefCell;
use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Kind {
    Gpio,
    Uart,
}

pub trait Dev {
    fn init(&'static self) -> Rv;
    fn kind(&self) -> Kind;
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result;

    fn eq(&self, dev: &'static dyn Dev) -> bool {
        return std::ptr::addr_eq(self, dev);
    }

    fn as_dev(&self) -> &(dyn Dev + Sync);
    fn as_gpio(&self) -> Option<&dyn gpio::Gpio> { None }
    fn as_uart(&self) -> Option<&dyn uart::Uart> { None }
}


struct DevInfo {
    dev: &'static (dyn Dev + Sync),
    status: Rv,
}

pub struct Mgr {
    devs: RefCell<Vec<DevInfo>>,
}

impl Mgr {
    pub fn new(climgr: &'static cli::Mgr) -> &'static Mgr {
        let devmgr = Box::leak(Box::new(Mgr {
            devs: RefCell::new(Vec::new()),
        }));

        climgr.reg("dev", "show devices", |cli, _args| {
            cli.print("devices:");
            for di in devmgr.devs.borrow().iter() {
                cli.printf(format_args!(
                    "- {:?}: {:?}: {:?}\n",
                    di.dev.kind(),
                    di.dev,
                    di.status
                ));
            }
            Rv::Ok
        });

        devmgr
    }
    
    pub fn add<T>(&self, dev: &'static T) -> &'static T 
        where T: Dev + Sync + ?Sized
    {
        self.devs.borrow_mut().push(DevInfo {
            dev: dev.as_dev(),
            status: Rv::ErrNotReady,
        });
        return dev;
    }

    pub fn init(&self) {
        log::inf!("devmgr init");
        for di in self.devs.borrow_mut().iter_mut() {
            println!("init {:?}", di.dev);
            di.status = di.dev.init();
        }
    }

    pub fn foreach_dev<F>(&self, f: F)
    where
        F: Fn(&'static (dyn Dev + Sync)),
    {
        for di in self.devs.borrow().iter() {
            f(di.dev);
        }
    }
}

impl fmt::Debug for (dyn Dev + Sync) {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.display(f);
    }
}
