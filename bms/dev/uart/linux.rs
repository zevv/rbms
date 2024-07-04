use std::fmt;
use std::sync::mpsc::SyncSender;
use std::sync::Mutex;
use std::thread;

use super::super::Dev;
use super::Stats;
use super::Uart;
use crate::bms::dev;
use crate::bms::evq::Event;
use crate::bms::evq::Evq;
use crate::bms::rv::Rv;
use libc;

const O_RDWR: i32 = 0x0002;

struct Dd {
    fd: i32,
    stats: Stats,
}

pub struct Linux {
    base: dev::Base,
    path: &'static str,
    sender: SyncSender<Event>,
    dd: Mutex<Dd>,
}

impl Linux {
    pub fn new(name: &'static str, evq: &Evq, path: &'static str) -> &'static (dyn Uart + Sync) {
        return Box::leak(Box::new(Linux {
            base: dev::Base {
                kind: super::Kind::Uart,
                name: name,
            },
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


    fn reader(&'static self) {
        let fd = self.dd.lock().unwrap().fd;
        loop {
            // Read one chunk of data and send it to the event queue
            let mut data: [u8; 8] = [0; 8];
            let len = unsafe { libc::read(fd, data.as_mut_ptr() as *mut libc::c_void, 8) };
            if len > 0 {

                if data[0..len as usize].contains(&0x03) {
                    unsafe { libc::abort(); }
                }

                let ev = Event::Uart {
                    dev: self,
                    data: data,
                    len: len as u8,
                };
                self.sender.send(ev).unwrap();

                // Update the stats
                let mut dd = self.dd.lock().unwrap();
                dd.stats.bytes_rx += len as u32;
            }
        }
    }
}

impl Dev for Linux {
    fn init(&'static self) -> Rv {
        let mut dd = self.dd.lock().unwrap();
        let path = std::ffi::CString::new(self.path).unwrap();
        
        dd.fd = unsafe { libc::open(path.as_ptr(), O_RDWR, 0) };

        unsafe {
            let mut tios: libc::termios = std::mem::zeroed();
            libc::tcgetattr(dd.fd, &mut tios);
            tios.c_lflag = 0;
            tios.c_cc[libc::VMIN] = 1;
            tios.c_cc[libc::VTIME] = 0;
            libc::tcflush(dd.fd, libc::TCIFLUSH);
            libc::tcsetattr(dd.fd, libc::TCSANOW, &tios);
        }

        thread::spawn(move || {
            self.reader();
        });

        Rv::Ok
    }

    fn base(&self) -> &dev::Base {
        &self.base
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
    fn write(&self, data: &[u8]) {
        let fd = self.dd.lock().unwrap().fd;
        let n = unsafe { libc::write(fd, data.as_ptr() as *const libc::c_void, data.len()) };
        if n >= 0 {
            self.dd.lock().unwrap().stats.bytes_tx += n as u32;
        }
    }

    fn get_stats(&self) -> Stats {
        let dd = self.dd.lock().unwrap();
        return dd.stats;
    }
}
