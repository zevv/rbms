
pub mod uart;
pub mod gpio;

use std::fmt;
use crate::bms::rv::Rv;

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
        return std::ptr::addr_eq(self, dev)
    }
}

struct DevInfo {
    dev: &'static (dyn Dev + Sync),
    status: Rv,
}

pub struct Mgr {
    devs: Vec<DevInfo>,
}

impl Mgr {
    pub fn new() -> Mgr {
        Mgr {
            devs: Vec::new(),
        }
    }

    pub fn add(&mut self, dev: &'static (dyn Dev + Sync)) -> &'static dyn Dev {
        self.devs.push(DevInfo {
            dev: dev,
            status: Rv::ErrNotReady,
        });
        return dev
    }

    pub fn init(&mut self) {
        for di in self.devs.iter_mut() {
            di.status = di.dev.init();
        }
    }

    pub fn dump(&self) {
        println!("devices:");
        for di in self.devs.iter() {
            println!("- {:?}: {:?}: {:?}", di.dev.kind(), di.dev, di.status);
        }
    }
}


impl fmt::Debug for (dyn Dev + Sync) {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.display(f);

    }
}

//impl fmt::Display for dyn Uart + Send + Sync {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        return self.display(f);
//    }
//}
