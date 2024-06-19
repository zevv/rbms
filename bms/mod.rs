

pub mod dev;
pub mod plat;
pub mod rv;

//use core::fmt::{self, Write};

//use crate::bms::plat::Plat;
//use crate::bms::plat::bms::Bms;


//extern "C" {
//    pub fn write(fd: i32, buf: *const u8, count: usize) -> isize;
//}

pub fn bms() {

    // drv::hello();
    // let a = 42;
    // let mut buf = [0u8; 128];
    // let mut buf = ByteMutWriter::new(&mut buf[..]);
    // buf.clear();
    // write!(buf, "The answer is: {}\n", a).unwrap();
    // unsafe {
    //     let p = buf.as_str().as_ptr();
    //     write(1, p, buf.len());
    // }
    //
    
    let mut devmgr = dev::Devmgr::new();

    //let plat = Box::new(plat::bms::linux::Linux{ value: 42 });
    let mut plat = plat::bms::linux::new(&mut devmgr);
    _ = plat.init(&mut devmgr);

    plat.devs().uart.uart0.borrow().write(b"=== Hello, world! ===\n");

    plat.devs().gpio.backlight.borrow_mut().set(true);


    // print typeof of plat
}

// pub struct ByteMutWriter<'a> {
//     buf: &'a mut [u8],
//     cursor: usize,
// }
// 
// impl<'a> ByteMutWriter<'a> {
//     pub fn new(buf: &'a mut [u8]) -> Self {
//         ByteMutWriter { buf, cursor: 0 }
//     }
// 
//     pub fn as_str(&self) -> &str {
//         core::str::from_utf8(&self.buf[..self.cursor]).unwrap()
//     }
// 
//     #[inline]
//     pub fn capacity(&self) -> usize {
//         self.buf.len()
//     }
// 
//     pub fn clear(&mut self) {
//         self.cursor = 0;
//     }
// 
//     pub fn len(&self) -> usize {
//         self.cursor
//     }
// 
// }
// 
// impl fmt::Write for ByteMutWriter<'_> {
//     fn write_str(&mut self, s: &str) -> fmt::Result {
//         let cap = self.capacity();
//         for (i, &b) in self.buf[self.cursor..cap]
//             .iter_mut()
//             .zip(s.as_bytes().iter())
//         {
//             *i = b;
//         }
//         self.cursor = usize::min(cap, self.cursor + s.as_bytes().len());
//         Ok(())
//     }
// 
// }
// 
