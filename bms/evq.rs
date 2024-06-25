
use std::cell::RefCell;
use std::sync::mpsc::{SyncSender, Receiver};
use crate::bms::cli;

use crate::bms::dev;
use crate::bms::rv::Rv;

// TODO there must be a better way

#[repr(u8)]
pub enum EvType {
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
    id: u8,
    cb: Box<dyn Fn(&Event)>,
    rv: Rv,
    count: u32,
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
                println!("{} {}", handler.id, handler.count);
            }
            Rv::Ok
        });

        evq
    }

    pub fn reg<F>(&self, cb: F) 
        where F: Fn(&Event) + 'static {
        let mut data = self.data.borrow_mut();
        data.handlers.push(
            Handler {
                cb: Box::new(cb),
                rv: Rv::Ok,
                id: 0,
                count: 0,
            });
    }

    pub fn reg_filter<F>(&self, filter_id: EvType, cb: F) 
        where F: Fn(&Event) + 'static {
        let mut data = self.data.borrow_mut();
        data.handlers.push(
            Handler {
                cb: Box::new(cb),
                rv: Rv::Ok,
                id: filter_id as u8,
                count: 0,
            });
    }
    
    pub fn run(&self) {
        loop {
            let event = self.rx.recv().unwrap();
            let id = unsafe { *<*const _>::from(&event).cast::<u8>() };
            let mut data = self.data.borrow_mut();
            for handler in data.handlers.iter_mut() {
                if handler.id == 0 || handler.id == id {
                    (handler.cb)(&event);
                    handler.count += 1;
                }
            }
        }
    }

    pub fn sender(&self) -> SyncSender<Event> {
        return self.tx.clone();
    }

    pub fn stop(&self) {
        //let mut state = self.data.borrow_mut();
        //println!("stop {}", state.running);
        //state.running = false;
    }
}

