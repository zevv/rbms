
use std::cell::RefCell;

struct Handler {
    cmd: String,
    cb: Box<dyn Fn(&str)>,
}

struct CliMgrState {
    handlers: Vec<Handler>,
}

pub struct CliMgr {
    state: RefCell<CliMgrState>,
}


impl CliMgr {

    pub fn new() -> &'static CliMgr {
        Box::leak(Box::new(CliMgr {
            state: RefCell::new(CliMgrState {
                handlers: Vec::new(),
            }),
        }))
    }
    
    pub fn reg<F>(&self, cmd: &str, cb: F) 
        where F: Fn(&str) + 'static {
        self.state.borrow_mut().handlers.push(Handler {
            cmd: cmd.to_string(),
            cb: Box::new(cb),
        });
    }

    pub fn add_cli<F>(&'static self, on_tx: F) -> &'static Cli 
        where F: Fn(u8) + 'static {

        return Box::leak(Box::new(Cli {
            state: RefCell::new(CliState {
                buf: ['\0'; 128],
                len: 0,
            }),
            on_tx: Box::new(on_tx),
            mgr: self,
        }));
    }
}


struct CliState {
    buf: [char; 128],
    len: usize,
}

pub struct Cli {
    state: RefCell<CliState>,
    on_tx: Box<dyn Fn(u8)>,
    mgr: &'static CliMgr,

}


impl Cli {

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
