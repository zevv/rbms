

pub mod gpio;
pub mod uart;

use super::rv::*;

pub trait Dev {
    fn init(&mut self) -> Result<(), Rv>;
}


pub struct Devmgr<'a> {
    dev: Vec<&'a dyn Dev>,
}


impl<'a> Devmgr<'a> {
    pub fn new() -> Devmgr<'a> {
        Devmgr {
            dev: Vec::new(),
        }
    }

    pub fn add(&mut self, dev: &'a dyn Dev) {
        self.dev.push(dev);
    }

    pub fn init(&mut self) -> Result<(), Rv> {
        for dev in self.dev.iter_mut() {
            // TODO
            //dev.init()?;
        }
        Ok(())
    }
}
