

pub mod evq;
pub mod rv;
pub mod dev;
pub mod plat;

use evq::Event;


mod cli {

    use std::cell::RefCell;

    struct Handler {
        cmd: String,
        cb: Box<dyn Fn(&str)>,
    }

    struct State {
        buf: [char; 128],
        len: usize,
        handlers: Vec<Handler>,
    }

    pub struct Cli {
        state: RefCell<State>,
        on_tx: Box<dyn Fn(u8)>,

    }

    pub fn new<F>(on_tx: F) -> &'static Cli 
        where F: Fn(u8) + 'static {

        return Box::leak(Box::new(Cli {
            state: RefCell::new(State {
                buf: ['\0'; 128],
                len: 0,
                handlers: Vec::new(),
            }),
            on_tx: Box::new(on_tx),
        }));
    }

    impl Cli {

        pub fn reg<F>(&self, cmd: &str, cb: F) 
            where F: Fn(&str) + 'static {
            let mut state = self.state.borrow_mut();
            state.handlers.push(Handler {
                cmd: cmd.to_string(),
                cb: Box::new(cb),
            });
        }

        pub fn handle_char(&self, c: char) {
            match c {
                '\n' => {
                    let mut state = self.state.borrow_mut();
                    self.handle_line(&state.buf[0..state.len]);
                    state.len = 0;
                }
                _ => {
                    let mut state = self.state.borrow_mut();
                    if state.len < state.buf.len() {
                        let len = state.len;
                        state.buf[len] = c;
                        state.len = len + 1;
                    }
                }
            }
        }

        fn split<'a>(&self, line: &'a[char], parts: &mut [&'a[char]; 8]) -> usize {
            let mut n = 0;
            let mut i = 0;
            let mut start = 0;
            let mut in_part = false;
            while i < line.len() {
                if line[i] == ' ' {
                    if in_part {
                        parts[n] = &line[start..i];
                        n += 1;
                        in_part = false;
                    }
                } else {
                    if !in_part {
                        start = i;
                        in_part = true;
                    }
                }
                i += 1;
            }
            if in_part {
                parts[n] = &line[start..i];
                n += 1;
            }
            return n;
        }

        fn handle_line(&self, line: &[char]) {
            println!("line: {:?}", line);
            let mut parts: [&[char]; 8] = [&[]; 8];
            let n = self.split(line, &mut parts);
            for i in 0..n {
                println!("part[{}]: {:?}", i, parts[i]);
            }
            
        }

        fn print(&self, s: &str) {
            for c in s.chars() {
                (self.on_tx)(c as u8);
            }
        }
    }
}


pub fn bms() {

    let evq = evq::Evq::new();
    let mut devmgr = dev::Mgr::new();

    #[cfg(feature = "linux")]
    let plat: &'static dyn plat::bms::Bms = plat::bms::linux::new(evq, &mut devmgr);
    
    #[cfg(feature = "nowos")]
    let plat: &'static dyn plat::bms::Bms = plat::bms::nowos::new(evq, &mut devmgr);

    let cli = cli::new(|c| {
        let buf = [ c as u8 ];
        plat.devs().uart.uart0.write(&buf);
    });

    cli.reg("help", |args| {
        println!("help: {:?}", args);
    });


    plat.init();

    devmgr.init();
    devmgr.dump();

    plat.devs().uart.uart0.write(b"=== Hello ===\n");
    plat.devs().gpio.backlight.set(true);
    
    evq.reg(|e| {
        match e {
            Event::Tick1Hz => {
                //println!("event: Tick1Hz");
                //println!("{:?}", plat.devs().uart.uart0.get_stats());
            }
            Event::Uart { dev, data, len } => {
                for i in 0..(*len as usize) {
                    cli.handle_char(data[i] as char);
                }
                //println!("event: Uart dev={:?} len={:?}, data={:?}", *dev as &(dyn dev::Dev + Sync), len, data);
            }
        }
    });


    evq.run();
}

