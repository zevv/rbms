
use super::Uart;
use super::super::Dev;
use std::thread;
use std::sync::Mutex;
use std::sync::mpsc::SyncSender;
use nix::fcntl::{OFlag, open};
use nix::unistd::{write, read};
use nix::sys::stat::Mode;
use crate::bms::evq::Event;
use crate::bms::evq::Evq;
use crate::bms::rv::Rv;

struct Dd {
    fd: i32,
}

struct Linux {
    path: &'static str,
    sender: SyncSender<Event>,
    dd: Mutex<Dd>,
}


pub fn new(evq: &Evq, path: &'static str) -> &'static dyn Uart {
    return Box::leak(Box::new(Linux {
        path: path,
        sender: evq.sender(),
        dd: Mutex::new(Dd { fd: 42 }),
    }));
}


impl Dev for Linux {
    fn init(&'static self) -> Rv {
        let fd = open(self.path, OFlag::O_RDWR, Mode::empty()).expect("Failed to open uart");
        self.dd.lock().unwrap().fd = fd;
        thread::spawn(move || {
            loop {
                let mut data: [u8; 8] = [0; 8];
                let len = read(fd, &mut data).expect("Failed to read");
                let ev = Event::Uart { 
                    //TODO dev: self,
                    data: data,
                    len: len as u8
                };
                self.sender.send(ev).unwrap();
            }
        });
        Rv::Ok
    }

    fn kind(&self) -> super::Kind {
        return super::Kind::Uart;
    }
}

impl Uart for Linux {
    fn write(&self, data: &[u8]) {
         _= write(self.dd.lock().unwrap().fd, data);
    }
}


