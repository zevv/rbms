

pub mod linux;

use super::Dev;
use super::Kind;

pub trait Uart : Dev {
    fn write(&self, data: &[u8]);
}


