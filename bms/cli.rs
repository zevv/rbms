use crate::bms::rv::Rv;
use crate::bms::log;
use std::cell::RefCell;

struct Handler {
    cmd: &'static str,
    usage: &'static str,
    cb: Box<dyn Fn(&Cli, &str) -> Rv>,
}

struct CliMgrState {
    handlers: Vec<Handler>,
}

pub struct CliMgr {
    state: RefCell<CliMgrState>,
}

impl CliMgr {
    pub fn new() -> &'static CliMgr {
        let climgr = Box::leak(Box::new(CliMgr {
            state: RefCell::new(CliMgrState {
                handlers: Vec::new(),
            }),
        }));

        climgr.reg("help", "show help", |cli, _args| {
            for h in cli.mgr.state.borrow().handlers.iter() {
                cli.print(h.cmd);
                cli.print(" - ");
                cli.print(h.usage);
                cli.print("\n");
            }
            Rv::Ok
        });

        linf!("Hello");

        climgr
    }

    pub fn reg<F>(&self, cmd: &'static str, usage: &'static str, cb: F)
    where
        F: Fn(&Cli, &str) -> Rv + 'static,
    {
        self.state.borrow_mut().handlers.push(Handler {
            cmd: cmd,
            usage: usage,
            cb: Box::new(cb),
        });
    }

    pub fn add_cli<F>(&'static self, on_tx: F) -> &'static Cli
    where
        F: Fn(u8) + 'static,
    {
        return Box::leak(Box::new(Cli {
            state: RefCell::new(CliState {
                buf: [0; 128],
                len: 0,
            }),
            on_tx: Box::new(on_tx),
            mgr: self,
        }));
    }

    pub fn handle_line(&self, cli: &Cli, parts: &[&str], _n: usize) {
        let mut rv = Rv::ErrInval;
        for h in self.state.borrow().handlers.iter() {
            if h.cmd == parts[0] {
                rv = (h.cb)(cli, parts[0]);
                break;
            }
        }
        println!(": {:?}", rv);
    }
}

struct CliState {
    buf: [u8; 128],
    len: usize,
}

pub struct Cli {
    state: RefCell<CliState>,
    on_tx: Box<dyn Fn(u8)>,
    mgr: &'static CliMgr,
}

impl Cli {
    pub fn handle_char(&self, c: u8) {
        match c as char {
            '\n' => {
                let mut state = self.state.borrow_mut();
                self.handle_line(std::str::from_utf8(&state.buf[0..state.len]).unwrap());
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

    fn split<'a>(&self, line: &'a str, parts: &mut [&'a str; 8]) -> usize {
        let mut n = 0;
        let mut i1 = 0;
        let mut i2 = 0;
        let mut inpart = false;
        for c in line.chars() {
            if c == ' ' {
                if inpart {
                    if n < parts.len() {
                        parts[n] = &line[i1..i2];
                        n += 1;
                    }
                    inpart = false;
                }
            } else {
                if !inpart {
                    i1 = i2;
                    inpart = true;
                }
            }
            i2 += 1;
        }
        if inpart {
            parts[n] = &line[i1..i2];
            n += 1;
        }
        return n;
    }

    fn handle_line(&self, line: &str) {
        if line.len() > 0 {
            let mut parts = [""; 8];
            let n = self.split(line, &mut parts);
            self.mgr.handle_line(self, &parts, n);
        }
        self.print("> ");
    }

    pub fn print(&self, s: &str) {
        for c in s.chars() {
            (self.on_tx)(c as u8);
        }
    }

}

