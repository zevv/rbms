
use std::thread;
use std::sync::Mutex;
use std::sync::mpsc::SyncSender;

use nix::fcntl::{OFlag, open};
use nix::unistd::{write, read};
use nix::sys::stat::Mode;

use crate::bms::evq::Event;
use crate::bms::evq::Evq;
use crate::bms::rv::Rv;
use super::Uart;
use super::Stats;
use super::super::Dev;

struct Dd {
    fd: i32,
    stats: Stats,
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
        dd: Mutex::new(Dd {
            fd: -1,
            stats: Stats {
                bytes_rx: 0,
                bytes_tx: 0,
            },
        }),
    }));
}


impl Dev for Linux {

    fn init(&'static self) -> Rv {
        let fd = open(self.path, OFlag::O_RDWR, Mode::empty()).expect("Failed to open uart");

        // Spawn a thread to read from the uart
        thread::spawn(move || {
            loop {
                // Read one chunk of data and send it to the event queue
                let mut data: [u8; 8] = [0; 8];
                let len = read(fd, &mut data).expect("Failed to read");
                let ev = Event::Uart { 
                    //TODO dev: self,
                    data: data,
                    len: len as u8
                };
                self.sender.send(ev).unwrap();

                // Update the stats
                let mut dd = self.dd.lock().unwrap();
                dd.stats.bytes_rx += len as u32;
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
}


impl Uart for Linux {

    // Write data to the uart
    fn write(&self, data: &[u8]) {
        let mut dd = self.dd.lock().unwrap();
         match write(dd.fd, data) {
            Ok(n) => {
                // On success, update the stats
                dd.stats.bytes_tx += n as u32;
            }
            Err(e) => {
                println!("Failed to write: {:?}", e);
            }
        }

    }

    fn get_stats(&self) -> Stats {
        let dd = self.dd.lock().unwrap();
        return dd.stats;
    }
}


