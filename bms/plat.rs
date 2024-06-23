
pub mod bms;

use super::rv::*;

pub trait Plat {
    fn init(&self) -> Rv;
}

