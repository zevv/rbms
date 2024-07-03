
#[cfg(feature = "linux")]
pub mod linux;

#[cfg(feature = "esp32")]
pub mod esp32;

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
}


