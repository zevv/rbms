
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

struct Dd {
    stats: Stats,
}

struct Esp32 {
    sender: SyncSender<Event>,
    dd: Mutex<Dd>,
}


pub fn new(evq: &Evq) -> &'static (dyn Uart + Sync) {
    return Box::leak(Box::new(Esp32 {
        sender: evq.sender(),
        dd: Mutex::new(Dd {
            stats: Stats {
                bytes_rx: 0,
                bytes_tx: 0,
            },
        }),
    }));
}


impl Dev for Esp32 {

    fn init(&'static self) -> Rv {
        Rv::Ok
    }

    fn kind(&self) -> super::Kind {
        return super::Kind::Uart;
    }
    
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "esp32");
    }
}

impl Uart for Esp32 {

    // Write data to the uart
    fn write(&self, data: &[u8]) {
    }

    fn get_stats(&self) -> Stats {
        let dd = self.dd.lock().unwrap();
        return dd.stats;
    }

}


