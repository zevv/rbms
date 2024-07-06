
use std::cell::RefCell;
use std::sync::mpsc::{SyncSender, Receiver};
use crate::bms::cli;
use crate::bms::log;

use crate::bms::dev;
use crate::bms::rv::Rv;

// TODO there must be a better way

#[repr(u8)]
pub enum EvType {
    Any = 0,
    Tick1Hz = 1,
    Uart = 2,
}

#[repr(u8)]
pub enum Event {
    Tick1Hz = 1,
    //Tick10Hz,
    Uart { 
        dev: &'static (dyn dev::uart::Uart + Sync),
        data: [u8; 8],
        len: u8,
    },
}

struct Handler {
    evtype: u8,
    cb: Box<dyn Fn(&Event)>,
    id: &'static str,
    rv: Rv,
    count: RefCell<u32>,
}

struct Data {
    running: bool,
}

pub struct Evq {
    rx: Receiver<Event>,
    tx: SyncSender<Event>,
    handlers: RefCell<Vec<Handler>>,
    data: RefCell<Data>,
}

//unsafe impl Sync for Evq {}

impl Evq {

    pub fn new(climgr: &'static cli::Mgr) -> &'static Evq {

        let (tx, rx) = std::sync::mpsc::sync_channel(32);
        let evq = Box::leak(Box::new(Evq {
            rx: rx,
            tx: tx,
            handlers: RefCell::new(Vec::new()),
            data: RefCell::new(Data { running: true }),
        }));

        climgr.reg("evq", "show event queue", |_cli, _args| {
            let hs = evq.handlers.borrow();
            for handler in hs.iter() {
                println!("{} {} {}", handler.evtype, handler.id, handler.count.borrow());
            }
            Rv::Ok
        });

        evq
    }

    pub fn reg<F>(&self, id: &'static str, cb: F) 
        where F: Fn(&Event) + 'static {
            self.reg_filter(id, EvType::Any, cb);
    }

    pub fn reg_filter<F>(&self, id: &'static str, evtype: EvType, cb: F) 
        where F: Fn(&Event) + 'static {
        self.handlers.borrow_mut().push(
            Handler {
                cb: Box::new(cb),
                id: id,
                rv: Rv::Ok,
                evtype: evtype as u8,
                count: 0.into()
            });
    }
    
    pub fn run(&self) {
        loop {
            let event = self.rx.recv().unwrap();
            let evtype = unsafe { *<*const _>::from(&event).cast::<u8>() };
            for handler in self.handlers.borrow_mut().iter() {
                if handler.evtype == 0 || handler.evtype == evtype {
                    (handler.cb)(&event);
                    *handler.count.borrow_mut() += 1;
                }
            }
            if self.data.borrow().running == false {
                break;
            }
        }
    }

    pub fn sender(&self) -> SyncSender<Event> {
        return self.tx.clone();
    }

    pub fn stop(&self) {
        let mut state = self.data.borrow_mut();
        state.running = false;
    }
}

