
pub mod uart;
pub mod gpio;

use crate::bms::rv::Rv;

#[derive(Debug)]
pub enum Kind {
    Gpio,
    Uart,
}

pub trait Dev {
    fn init(&'static self) -> Rv;
    fn kind(&self) -> Kind;
}

struct DevInfo {
    dev: &'static dyn Dev,
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

    pub fn add(&mut self, dev: &'static dyn Dev) {
        self.devs.push(DevInfo {
            dev: dev,
            status: Rv::ErrNotReady,
        });
    }

    pub fn init(&mut self) {
        for di in self.devs.iter_mut() {
            di.status = di.dev.init();
        }
    }

    pub fn dump(&self) {
        for di in self.devs.iter() {
            println!("Dev: {:?} {:?}", di.dev.kind(), di.status);
        }
    }
}

