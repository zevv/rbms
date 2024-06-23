
use std::cell::RefCell;
use std::sync::mpsc::{SyncSender, Receiver};

use crate::bms::dev;

pub enum Event {
    Tick1Hz,
    //Tick10Hz,
    Uart { 
        dev: &'static (dyn dev::uart::Uart + Send + Sync),
        data: [u8; 8],
        len: u8,
    },
}

struct Handler {
    cb: Box<dyn Fn(&Event)>,
}

pub struct Evq {
    rx: Receiver<Event>,
    tx: SyncSender<Event>,
    handlers: RefCell<Vec<Handler>>,
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

    pub fn reg<F>(&self, cb: F) 
        where F: Fn(&Event) + 'static
    {
        let mut handlers = self.handlers.borrow_mut();
        handlers.push(Handler { cb: Box::new(cb) });
        
    }
    
    pub fn run(&self) {
        loop {
            let event = self.rx.recv().unwrap();
            for handler in self.handlers.borrow().iter() {
                (handler.cb)(&event);
                
            }
        }
    }

    pub fn sender(&self) -> SyncSender<Event> {
        return self.tx.clone();
    }
}

