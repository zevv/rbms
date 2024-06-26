
use std::cell::RefCell;
use std::sync::mpsc::{SyncSender, Receiver};
use crate::bms::cli;

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
    handlers: Vec<Handler>,
}

pub struct Evq {
    rx: Receiver<Event>,
    tx: SyncSender<Event>,
    data: RefCell<Data>,
}

//unsafe impl Sync for Evq {}

impl Evq {

    pub fn new(climgr: &'static cli::Mgr) -> &'static Evq {

        let (tx, rx) = std::sync::mpsc::sync_channel(32);
        let evq = Box::leak(Box::new(Evq {
            rx: rx,
            tx: tx,
            data: RefCell::new(Data {
                running: true,
                handlers: Vec::new(),
            })
        }));

        climgr.reg("evq", "show event queue", |_cli, _args| {
            let data = evq.data.borrow();
            for handler in data.handlers.iter() {
                println!("{} {}", handler.id, handler.count.borrow());
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
        let mut data = self.data.borrow_mut();
        data.handlers.push(
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
            let data = self.data.borrow();
            for handler in data.handlers.iter() {
                if handler.evtype == 0 || handler.evtype == evtype {
                    (handler.cb)(&event);
                    *handler.count.borrow_mut() += 1;
                }
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

