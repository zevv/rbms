
#![feature(trait_upcasting)]
//#![allow(incomplete_features)]
//#![no_std]
//#![no_main]
//#[panic_handler]
//fn panic(_info: &core::panic::PanicInfo) -> ! {
//    loop {}
//}
//#[no_mangle]
//pub fn rust_eh_personality() {
//}

mod bms;
use crate::bms::bms;

//#[no_mangle]
fn main() {

    bms().expect("BMS failed");
}


