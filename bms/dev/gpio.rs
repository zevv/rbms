
pub mod dummy;

use super::Dev;

use crate::bms::rv::*;

pub trait Gpio : Dev {

    fn set(&self, val: bool) -> Result<(), Rv>;
    fn get(&self) -> Result<bool, Rv>;

}



