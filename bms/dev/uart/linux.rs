
use std::thread;
use std::sync::Mutex;
use std::sync::mpsc::SyncSender;
use std::fmt;

use crate::bms::evq::Event;
use crate::bms::evq::Evq;
use crate::bms::rv::Rv;
use super::Uart;
use super::Stats;
use super::super::Dev;

extern "C" {
    fn open(path: *const i8, flags: i32, mode: i32) -> i32;
    fn write(fd: i32, data: *const u8, len: usize) -> i32;
    fn read(fd: i32, data: *mut u8, len: usize) -> i32;
}

const O_RDWR: i32 = 0x0002;

struct Dd {
    fd: i32,
    stats: Stats,
}

pub struct Linux {
    name: &'static str,
    path: &'static str,
    sender: SyncSender<Event>,
    dd: Mutex<Dd>,
}


impl Linux {
    pub fn new(name: &'static str, evq: &Evq, path: &'static str) -> &'static (dyn Uart + Sync) {
        return Box::leak(Box::new(Linux {
            name: name,
            path: path,
            sender: evq.sender(),
            dd: Mutex::new(Dd {
                fd: -1,
                stats: Stats {
                    bytes_rx: 0,
                    bytes_tx: 0,
                },
            }),
        }));
    }
}

impl Dev for Linux {

    fn init(&'static self) -> Rv {

        let path = std::ffi::CString::new(self.path).unwrap();
        let fd = unsafe {
            open(path.as_ptr(), O_RDWR, 0)
        };

        // Spawn a thread to read from the uart
        thread::spawn(move || {
            loop {
                // Read one chunk of data and send it to the event queue
                let mut data: [u8; 8] = [0; 8];
                let len = unsafe {
                    read(fd, data.as_mut_ptr(), 8)
                };
                if len > 0 {
                    let ev = Event::Uart { 
                        dev: self,
                        data: data,
                        len: len as u8
                    };
                    self.sender.send(ev).unwrap();

                    // Update the stats
                    let mut dd = self.dd.lock().unwrap();
                    dd.stats.bytes_rx += len as u32;
                }
            }
        });

        // Save the file descriptor
        let mut dd = self.dd.lock().unwrap();
        dd.fd = fd;
        Rv::Ok
    }

    fn kind(&self) -> super::Kind {
        return super::Kind::Uart;
    }

    fn get_name(&self) -> &str {
        self.name
    }
    
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "linux@{}", self.path)
    }

    fn as_dev(&self) -> &(dyn Dev + Sync) {
        self
    }

    fn as_uart(&self) -> Option<&dyn Uart> {
        Some(self)
    }
}


impl Uart for Linux {

    // Write data to the uart
    fn write(&self, data: &[u8]) {
        let fd = self.dd.lock().unwrap().fd;
        let n = unsafe {
            write(fd, data.as_ptr(), data.len())
        };
        if n >= 0 {
            self.dd.lock().unwrap().stats.bytes_tx += n as u32;
        }
    }

    fn get_stats(&self) -> Stats {
        let dd = self.dd.lock().unwrap();
        return dd.stats;
    }

}


