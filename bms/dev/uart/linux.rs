

use std::rc::Rc;
use std::cell::RefCell;
use super::super::Dev;
use super::Uart;
use crate::bms::rv::*;
//use std::os::unix::io::RawFd;
use nix::sys::stat::Mode;
use nix::unistd::{write}; // , read, close};
//use nix::sys::termios::{self, Termios, InputFlags, LocalFlags, ControlFlags, OutputFlags, SetArg, tcgetattr, tcsetattr, cfsetspeed, BaudRate};
use nix::sys::termios::{tcgetattr};

use nix::fcntl::{OFlag, open};

pub struct Linux {
    dev: &'static str,
    fd: i32,
}

pub fn new(dev: &'static str) -> Rc<RefCell<dyn Uart>> {
    Rc::new(RefCell::new(Linux {
        dev: dev,
        fd: -1,
    }))
}


impl Linux {
    pub fn flap(&mut self) {
    }
}


impl Dev for Linux {

    // TODO: super?
    fn kind(&self) -> super::super::Kind {
        super::super::Kind::Uart
    }

    fn init(&mut self) -> Result<(), Rv> {

        match open(self.dev, OFlag::O_RDWR | OFlag::O_NOCTTY, Mode::empty()) {
            Ok(fd) => { self.fd = fd; }
            Err(_) => { return Err(Rv::ErrIo); }
        }

        let _tios = tcgetattr(self.fd);

        self.flap();
        println!("dev::uart::Linux.init() fd={}", self.fd);
        Ok(())
    }
}


impl Uart for Linux {
    fn write(&self, val: &[u8]) -> Result<(), Rv> {
        match write(self.fd, val) {
            Ok(_) => { Ok(()) }
            Err(_) => { return Err(Rv::ErrIo); }
        }
    }
}

