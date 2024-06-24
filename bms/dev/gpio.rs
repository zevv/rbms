
pub mod dummy;

#[cfg(feature = "esp32")]
pub mod esp32;

use super::Dev;
use crate::bms::rv::Rv;

pub trait Gpio : Dev {
    fn set(&self, state: bool) -> Rv;
}

