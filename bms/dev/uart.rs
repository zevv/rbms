
pub mod linux;

use std::fmt;

use super::Dev;
use super::Kind;

#[derive(Clone, Copy, Debug)]
pub struct Stats {
    pub bytes_rx: u32,
    pub bytes_tx: u32,
}

pub trait Uart : Dev {
    fn write(&self, data: &[u8]);
    fn get_stats(&self) -> Stats;
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result;
}


impl fmt::Display for dyn Uart + Send + Sync {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.display(f)
    }
}


