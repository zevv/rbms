
use std::thread;
use std::sync::Mutex;
use std::sync::mpsc::SyncSender;
use std::fmt;
use esp_idf_hal::delay::BLOCK;

use esp_idf_hal::uart::*;

use crate::bms::evq::Event;
use crate::bms::evq::Evq;
use crate::bms::rv::Rv;
use crate::bms::log;
use super::Uart;
use super::Stats;
use super::super::Dev;
use esp_idf_hal::gpio;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripheral::Peripheral;

struct Dd {
    stats: Stats,
}

struct Esp32 {
    dev: UartDriver<'static>,
    sender: SyncSender<Event>,
    dd: Mutex<Dd>,
}


pub fn new<U: uart::Uart, TI: InputPin, TO: OutputPin>(evq: &Evq, uart: impl Peripheral<P = U> + 'static, tx: TO, rx: TI) -> &'static (dyn Uart + Sync)
{

    let cfg = config::Config::new().baudrate(Hertz(115_200));
    let uartdev: UartDriver = UartDriver::new(
        uart,
        tx,
        rx,
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &cfg,
        ).unwrap();

    return Box::leak(Box::new(Esp32 {
        sender: evq.sender(),
        dev: uartdev,
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

        thread::spawn(move || {
            loop {
                let mut buf = [0_u8; 8];
                let n = self.dev.read(&mut buf, 1).unwrap();
                if n > 0 {
                    let ev = Event::Uart {
                        data: buf,
                        len: n as u8,
                        dev: self,
                    };
                    self.sender.send(ev);
                }
            }
        });


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
        let mut dd = self.dd.lock().unwrap();
        dd.stats.bytes_tx += data.len() as u32;
        self.dev.write(data);
    }

    fn get_stats(&self) -> Stats {
        let dd = self.dd.lock().unwrap();
        return dd.stats;
    }

}


