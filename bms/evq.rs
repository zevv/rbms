
use std::cell::RefCell;
use std::sync::mpsc::{SyncSender, Receiver};

use crate::bms::dev;
use crate::bms::rv::Rv;

pub enum Event {
    Tick1Hz,
    //Tick10Hz,
    Uart { 
        dev: &'static (dyn dev::uart::Uart + Sync),
        data: [u8; 8],
        len: u8,
    },
}

struct Handler {
    cb: Box<dyn Fn(&Event)>,
    rv: Rv,
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

    pub fn new() -> &'static Evq {
        let (tx, rx) = std::sync::mpsc::sync_channel(32);
        return Box::leak(Box::new(Evq {
            rx: rx,
            tx: tx,
            data: RefCell::new(Data {
                running: true,
                handlers: Vec::new(),
            })
        }));
    }

    pub fn reg<F>(&self, cb: F) 
        where F: Fn(&Event) + 'static
    {
        let mut state = self.data.borrow_mut();
        state.handlers.push(
            Handler {
                cb: Box::new(cb),
                rv: Rv::Ok,
            });
    }
    
    pub fn run(&self) {
        loop {
            let event = self.rx.recv().unwrap();
            let handlers = &self.data.borrow().handlers;
            for handler in handlers.iter() {
                (handler.cb)(&event);
            }
        }
    }

    pub fn sender(&self) -> SyncSender<Event> {
        return self.tx.clone();
    }

    pub fn stop(&self) {
        let mut state = self.data.borrow_mut();
        println!("stop {}", state.running);
        //state.running = false;
    }
}

