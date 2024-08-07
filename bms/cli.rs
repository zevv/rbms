use crate::bms::log;
use crate::bms::rv::Rv;
use libc;
use std::cell::RefCell;
use std::io::Write;

struct Handler {
    cmd: &'static str,
    usage: &'static str,
    cb: Box<dyn Fn(&Cli, &[&str]) -> Rv>,
}

struct MgrState {
    handlers: Vec<Handler>,
}

pub struct Mgr {
    state: RefCell<MgrState>,
}

#[macro_export]
macro_rules! ff { ($($arg:tt)*) => (format_args!($($arg)*)); }
pub(crate) use ff;

impl Mgr {
    pub fn new() -> &'static Mgr {
        let climgr = Box::leak(Box::new(Mgr {
            state: RefCell::new(MgrState {
                handlers: Vec::new(),
            }),
        }));

        climgr.reg("help", "show help", |cli, _args| {
            for h in cli.mgr.state.borrow().handlers.iter() {
                cli.printf(ff!("{} - {}\n", h.cmd, h.usage));
            }
            Rv::Ok
        });

        log::inf!("Hello");

        climgr
    }

    pub fn reg<F>(&self, cmd: &'static str, usage: &'static str, cb: F)
    where
        F: Fn(&Cli, &[&str]) -> Rv + 'static,
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
                escape: 0,
                pos: 0,
            }),
            on_tx: Box::new(on_tx),
            mgr: self,
        }));
    }

    pub fn handle_line(&self, cli: &Cli, parts: &[&str]) {
        let mut rv = Rv::ErrInval;
        if parts.len() > 0 {
            for h in self.state.borrow().handlers.iter() {
                if h.cmd == parts[0] {
                    rv = (h.cb)(cli, &parts[1..]);
                    break;
                }
            }
            if rv != Rv::Ok {
                cli.printf(format_args!(": {:?}\n", rv));
            }
        }
    }
}

struct CliState {
    buf: [u8; 128],
    len: usize,
    escape: usize,
    pos: usize,
}

pub struct Cli {
    state: RefCell<CliState>,
    on_tx: Box<dyn Fn(u8)>,
    mgr: &'static Mgr,
}

impl Cli {
    pub fn handle_char(&self, c: u8) {
        let mut state = self.state.borrow_mut();
        match state.escape {
            0 => match c {
                1 => {
                    // Ctrl-A: move to beginning of line
                    state.pos = 0;
                }
                4 => {
                    // Ctrl-D: delete character under cursor
                    if state.pos < state.len {
                        for i in state.pos..state.len {
                            state.buf[i] = state.buf[i + 1];
                        }
                        state.len -= 1;
                    }
                }
                5 => {
                    // Ctrl-E: move to end of line
                    state.pos = state.len;
                }
                8 | 127 => {
                    // Backspace
                    if state.pos > 0 {
                        state.pos -= 1;
                        for i in state.pos..state.len {
                            state.buf[i] = state.buf[i + 1];
                        }
                        state.len -= 1;
                    }
                }
                27 => {
                    // Escape
                    state.escape = 1;
                }
                10 | 13 => {
                    // Enter
                    self.write("\r\n".as_bytes());
                    self.handle_line(std::str::from_utf8(&state.buf[0..state.len]).unwrap());
                    state.len = 0;
                    state.pos = 0;
                }
                _ => {
                    // Insert character at cursor
                    if state.len < state.buf.len() {
                        let len = state.len;
                        let pos = state.pos;
                        for i in (pos..len).rev() {
                            state.buf[i + 1] = state.buf[i];
                        }
                        state.buf[pos] = c;
                        state.len = len + 1;
                        state.pos = state.pos + 1;
                    }
                }
            },

            1 => {
                if c == 91 { // "["
                    state.escape = 2;
                } else {
                    state.escape = 0;
                }
            }

            2 => {
                match c {
                    67 => { // Left arrow
                        if state.pos < state.len {
                            state.pos += 1;
                            self.write("\x1b[C".as_bytes());
                        }
                    }
                    68 => { // Right arrow
                        if state.pos > 0 {
                            state.pos -= 1;
                            self.write("\x1b[D".as_bytes());
                        }
                    }
                    _ => {
                        state.escape = 0;
                    }
                }
                state.escape = 0;
            }

            _ => {
                state.escape = 0;
            }
        }

        self.write(b"\r\x1b[1mbms>\x1b[0m ");
        self.write(&state.buf[0..state.len]);
        self.write(b"\x1b[K");
        if state.pos < state.len {
            self.printf(std::format_args!("\x1b[{}D", state.len - state.pos));
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
            self.mgr.handle_line(self, &parts[0..n]);
        }
        self.print("> ");
    }

    pub fn print(&self, s: &str) {
        for c in s.chars() {
            (self.on_tx)(c as u8);
        }
    }

    pub fn write(&self, buf: &[u8]) {
        for c in buf.iter() {
            (self.on_tx)(*c);
        }
    }

    pub fn printf(&self, args: std::fmt::Arguments) {
        let mut linebuf = [0u8; 128];
        let mut slice = &mut linebuf[..];
        _ = write!(slice, "{}", args);
        let n = slice.as_ptr() as usize - linebuf.as_ptr() as usize;
        let line = &linebuf[..n];
        self.write(line);
    }
}
