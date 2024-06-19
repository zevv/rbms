#![no_std]
#![no_main]

mod bms;

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, peripherals::Peripherals, prelude::*, system::SystemControl,
    gpio::{self, Event, Gpio9, Input, Io, Level, Output, Pull},
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = Output::new(io.pins.gpio15, Level::High);
    led.set_high();

    loop {
        log::info!("Hello world!");
        //bms();
        delay.delay(500.millis());
    }
}


