


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

    // Register device to the device manager.
    pub fn add(&mut self, dev: Rc<RefCell<dyn Dev>>) -> Rc<RefCell<dyn Dev>> {
        let di = DevInfo {
            dev: dev.clone(),
            status: Rv::ErrNotready,
        };
        self.dev.push(di);
        dev
    }

    // Initialize all devices. A device might not be able to initialize because it depends on other
    // devices; it this case it returns ErrNotready. For sake of simplicity, just attempt to
    // initialize all devices in a loop until all are ready instead of mainaining a tree of
    // dependencies.
    pub fn init(&mut self) -> Result<(), Rv> {
        println!("devmgr.init()");
        for _ in 0..10 {
            let mut some_not_ready = false;
            for di in self.dev.iter_mut() {
                match di.dev.borrow_mut().init() {
                    Ok(_) => { di.status = Rv::Ok; },
                    Err(e) => { di.status = e; }
                }
                if di.status == Rv::ErrNotready {
                    some_not_ready = true;
                }
            }
            if !some_not_ready {
                break;
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

