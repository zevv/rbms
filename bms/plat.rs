
pub mod bms;

use super::rv::*;
use crate::bms::dev::Devmgr;

pub trait Plat {
    fn init(&mut self, devmgr: &mut Devmgr) -> Result<(), Rv>;
}

