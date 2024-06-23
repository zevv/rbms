
use std::cell::RefCell;
use std::sync::mpsc::{SyncSender, Receiver};

use crate::bms::dev;

pub enum Event {
    Tick1Hz,
    //Tick10Hz,
    Uart { 
        //TODO dev: &'static dev::uart::Uart,Tu
        data: [u8; 8],
        len: u8,
    },
}


pub struct Evq {
    rx: Receiver<Event>,
    tx: SyncSender<Event>,
    handlers: RefCell<Vec<fn(&Event)>>,
}

impl Evq {

    pub fn new() -> &'static Evq {
        let (tx, rx) = std::sync::mpsc::sync_channel(32);
        return Box::leak(Box::new(Evq {
            rx: rx,
            tx: tx,
            handlers: RefCell::new(Vec::new()),
        }));
    }

    pub fn reg(&self, cb: fn(&Event)) {
        self.handlers.borrow_mut().push(cb);
    }
    
    pub fn run(&self) {
        loop {
            let event = self.rx.recv().unwrap();
            for h in self.handlers.borrow().iter() {
                h(&event);
            }
        }
    }

    pub fn sender(&self) -> SyncSender<Event> {
        return self.tx.clone();
    }
}

