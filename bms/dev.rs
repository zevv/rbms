pub mod gpio;
pub mod uart;

use crate::bms::log;
use crate::bms::rv::Rv;
use std::cell::RefCell;
use std::fmt;

#[derive(Debug)]
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
}

struct DevInfo {
    dev: &'static (dyn Dev + Sync),
    status: Rv,
}

pub struct Mgr {
    devs: RefCell<Vec<DevInfo>>,
}

impl Mgr {
    pub fn new() -> &'static Mgr {
        Box::leak(Box::new(Mgr {
            devs: RefCell::new(Vec::new()),
        }))
    }

    pub fn add(&self, dev: &'static (dyn Dev + Sync)) -> &'static dyn Dev {
        self.devs.borrow_mut().push(DevInfo {
            dev: dev,
            status: Rv::ErrNotReady,
        });
        return dev;
    }

    pub fn init(&self) {
        for di in self.devs.borrow_mut().iter_mut() {
            di.status = di.dev.init();
        }
    }

    pub fn dump(&self) {
        log::inf("devices:");
        for di in self.devs.borrow().iter() {
            println!("- {:?}: {:?}: {:?}", di.dev.kind(), di.dev, di.status);
        }
    }
}

impl fmt::Debug for (dyn Dev + Sync) {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.display(f);
    }
}
