

use std::rc::Rc;
use std::cell::RefCell;
use std::thread;
use std::sync::mpsc::SyncSender;

use nix::sys::stat::Mode;
use nix::unistd::{write};
use nix::sys::termios::{tcgetattr};

use crate::bms::evq::Event;
use crate::bms::rv::Rv;
use crate::bms::dev;
use crate::bms::dev::Dev;
use crate::bms::dev::uart::Uart;

use nix::fcntl::{OFlag, open};

pub struct Linux {
    dev: &'static str,
    fd: i32,
    sender: SyncSender<Event>,
}

pub fn new(sender: SyncSender<Event>, dev: &'static str) -> Rc<RefCell<dyn Uart>> {
    Rc::new(RefCell::new(Linux {
        dev: dev,
        fd: -1,
        sender: sender,
    }))
}


fn ticker(sender: SyncSender<Event>)
{
    //thread::spawn(|| {
    //    loop {
    //        thread::sleep(std::time::Duration::from_millis(1000));
    //        sender.send(Event::Tick1Hz {});
    //    }
    //});
}


impl Linux {
    pub fn flap(&mut self) {
    }
}


impl Dev for Linux {

    fn kind(&self) -> dev::Kind {
        dev::Kind::Uart
    }

    fn init(&mut self) -> Result<(), Rv> {

        match open(self.dev, OFlag::O_RDWR, Mode::empty()) {
            Ok(fd) => { self.fd = fd; }
            Err(_) => { return Err(Rv::ErrIo); }
        }

        let _tios = tcgetattr(self.fd);

        self.flap();
        println!("dev::uart::Linux.init() fd={}", self.fd);

        let s = self.sender.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(std::time::Duration::from_millis(1000));
                s.send(Event::Tick10Hz {}).unwrap();
            }
        });

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


