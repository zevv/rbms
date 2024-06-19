
#![feature(trait_upcasting)]
#![allow(incomplete_features)]

// #![no_std]
// #![no_main]
// #[panic_handler]
// fn panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {}
// }
// #[no_mangle]
// pub fn rust_eh_personality() {
// }

mod bms;
//
//mod uart {
//
//    pub trait Driver {
//        fn init(&self);
//        fn write(&self, data: u8);
//    }
//
//    pub struct Device {
//        pub drv: &'static dyn Driver,
//    }
//
//    impl Device {
//        pub fn new(drv: &'static dyn Driver) -> Self {
//            Self { drv }
//        }
//
//        pub fn init(&self) {
//            self.drv.init();
//        }
//
//        pub fn write(&self, data: u8) {
//            self.drv.write(data);
//        }
//
//    }
//
//    pub mod dummy {
//        use super::Driver;
//
//        pub struct DummyDriver {
//            pub value: bool,
//        }
//
//        impl Driver for DummyDriver {
//            fn init(&self) {
//            }
//
//            fn write(&self, data: u8) {
//                unsafe {
//                    extern "C" {
//                        pub fn putchar(c: u8);
//                    }
//                    putchar(data);
//                }
//            }
//        }
//
//    }
//
//}
//
use crate::bms::bms;

// glbal dummy driver

//const DUMMY_DRIVER: uart::dummy::DummyDriver = uart::dummy::DummyDriver { value: false };

//#[no_mangle]
fn main() {

    //let console = uart::Device::new(&DUMMY_DRIVER);
    //console.init();
    //hola(&console);
    bms();
}


