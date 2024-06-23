
pub mod dummy;

use super::Dev;
use crate::bms::rv::Rv;

pub trait Gpio : Dev {
    fn set(&self, state: bool) -> Rv;
}

