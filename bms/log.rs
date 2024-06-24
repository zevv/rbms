
use std::sync::Mutex;
use crate::bms::dev;

struct Logger {
    uart: Mutex<Option<&'static (dyn dev::uart::Uart + Sync)>>,
}

extern "C" {
    fn time(timep: *mut i64) -> i64;
    fn gmtime(timep: *const i64, result: *const u8) -> *const u8;
    fn strftime(s: *mut u8, maxsize: usize, format: *const i8, timeptr: *const u8) -> usize;
}


fn logf(tag: &str, color: &str, line: &str) {
    let mut buf: [u8; 17] = [0; 17];
    unsafe {
        let t = time(0 as *mut i64);
        let tm = gmtime(&t, 0 as *const u8);
        let fmt = "%y-%m-%d %H:%M:%S".as_ptr() as *const i8;
        strftime(buf.as_mut_ptr(), 64, fmt, tm);
    }
    
    let uart = LOGGER.uart.lock().unwrap().unwrap();
    uart.write(color.as_bytes());
    uart.write(&buf);
    uart.write(b" ");
    uart.write(tag.as_bytes());
    uart.write(b" ");
    uart.write(line.as_bytes());
    uart.write(b"\x1b[0m\n");
}

static LOGGER: Logger = Logger {
    uart: Mutex::new(None),
};

pub fn set_console(uart: &'static (dyn dev::uart::Uart + Sync)) {
    LOGGER.uart.lock().unwrap().replace(uart);
}

#[allow(dead_code)]
pub fn dmp(line: &str) { logf(&"dmp", &"\x1b[33m", line); }
#[allow(dead_code)]
pub fn dbg(line: &str) { logf(&"dbg", &"\x1b[22m", line); }
#[allow(dead_code)]
pub fn inf(line: &str) { logf(&"inf", &"\x1b[1m", line); }
#[allow(dead_code)]
pub fn wrn(line: &str) { logf(&"wrn", &"\x1b[33m", line); }
#[allow(dead_code)]
pub fn err(line: &str) { logf(&"err", &"\x1b[31m", line); }
#[allow(dead_code)]
pub fn tst(line: &str) { logf(&"tst", &"\x1b[7m", line); }

