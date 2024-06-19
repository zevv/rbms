


pub mod gpio;
pub mod uart;

use std::rc::Rc;
use std::cell::RefCell;
use super::rv::*;

#[derive(Debug)]
pub enum Kind {
    Gpio,
    Uart,
}

pub trait Dev {
    fn init(&mut self) -> Result<(), Rv>;
    fn kind(&self) -> Kind;
}


struct DevInfo {
    dev: Rc<RefCell<dyn Dev>>,
    status: Rv,
}

pub struct Devmgr {
    dev: Vec<DevInfo>
}


impl Devmgr {
    pub fn new() -> Devmgr {
        Devmgr {
            dev: Vec::new(),
        }
    }

    pub fn add(&mut self, dev: Rc<RefCell<dyn Dev>>) {
        let di = DevInfo {
            dev: dev,
            status: Rv::Ok,
        };
        self.dev.push(di);
    }

    pub fn init(&mut self) -> Result<(), Rv> {
        println!("devmgr.init()");
        for di in self.dev.iter_mut() {
            match di.dev.borrow_mut().init() {
                Ok(_) => { di.status = Rv::Ok; }
                Err(e) => { di.status = e; }
            }
        }
        Ok(())
    }

    pub fn dump(&self) -> Result<(), Rv> {
        for di in self.dev.iter() {
            println!("  {:?}: {}", di.dev.borrow().kind(), di.status);
        }
        Ok(())
    }
}
