
use std::sync::Mutex;
use crate::bms::dev;
use std::io::Write;

extern "C" {
    fn time(timep: *mut i64) -> i64;
    fn gmtime(timep: *const i64, result: *const u8) -> *const u8;
    fn strftime(s: *mut u8, maxsize: usize, format: *const i8, timeptr: *const u8) -> usize;
}

pub enum Level { Dmp, Dbg, Inf, Wrn, Err, Tst, }

struct LevelInfo {
    tag: &'static str,
    color: &'static str,
}

static LEVEL_INFO: [LevelInfo; 6] = [
    LevelInfo { tag: "dmp", color: "\x1b[33m" },
    LevelInfo { tag: "dbg", color: "\x1b[22m" },
    LevelInfo { tag: "inf", color: "\x1b[1m" },
    LevelInfo { tag: "wrn", color: "\x1b[33m" },
    LevelInfo { tag: "err", color: "\x1b[31m" },
    LevelInfo { tag: "tst", color: "\x1b[7m" },
];

struct Data {
    uart: Option<&'static (dyn dev::uart::Uart + Sync)>,
}

struct Logger {
    data: Mutex<Data>,
}

static LOGGER: Logger = Logger {
    data: Mutex::new(Data { uart: None }),
};

pub fn set_console(uart: &'static (dyn dev::uart::Uart + Sync)) {
    let mut data = LOGGER.data.lock().unwrap();
    data.uart = Some(uart);
}

pub fn logf(level: Level, path: &str, args: std::fmt::Arguments) {
    // format string into fixed size buffer, truncate if too long.
    let mut linebuf = [0u8; 128];
    let mut slice = &mut linebuf[..];
    let _ = write!(slice, "{}", args);
    let n = slice.as_ptr() as usize - linebuf.as_ptr() as usize;
    let line = &linebuf[0..n];

    // format timestamp in fixed size buffer.
    let mut timebuf: [u8; 17] = [0; 17];
    unsafe {
        let t = time(0 as *mut i64);
        let tm = gmtime(&t, 0 as *const u8);
        let fmt = "%y-%m-%d %H:%M:%S".as_ptr() as *const i8;
        strftime(timebuf.as_mut_ptr(), 64, fmt, tm);
    }

    // emit log message.
    let data = LOGGER.data.lock().unwrap();

    let li = &LEVEL_INFO[level as usize];
    match data.uart {
        Some(uart) => {
            uart.write(li.color.as_bytes());
            uart.write(&timebuf);
            uart.write(b" ");
            uart.write(li.tag.as_bytes());
            uart.write(b" ");
            let l = path.len() - 5;
            uart.write(path[l..].as_bytes());
            uart.write(b" ");
            uart.write(&linebuf);
            uart.write(b"\x1b[0m\n");
        }
        None => {}
    }
}


#[macro_export]
macro_rules! ldmp { ($($arg:tt)*) => (log::logf(log::Level::Dmp, module_path!(), format_args!($($arg)*))); }
#[macro_export]
macro_rules! ldbg { ($($arg:tt)*) => (log::logf(log::Level::Dbg, module_path!(), format_args!($($arg)*))); }
#[macro_export]
macro_rules! linf { ($($arg:tt)*) => (log::logf(log::Level::Inf, module_path!(), format_args!($($arg)*))); }
#[macro_export]
macro_rules! lwrn { ($($arg:tt)*) => (log::logf(log::Level::Wrn, module_path!(), format_args!($($arg)*))); }
#[macro_export]
macro_rules! lerr { ($($arg:tt)*) => (log::logf(log::Level::Err, module_path!(), format_args!($($arg)*))); }


