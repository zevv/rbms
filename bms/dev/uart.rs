
pub mod linux;

use super::Dev;

use crate::bms::rv::*;

pub trait Uart : Dev {

    fn write(&self, data: &[u8]) -> Result<(), Rv>;

}
