//! Collectors gather data from sites
//!

pub mod errors;
pub mod fantia_jp;
pub mod only_fans;
pub mod reddit;

use crossbeam_channel::Sender;

use crate::download::DownloadItem;
use errors::*;

pub async fn run_collector(c: impl Collector, send_chan: Sender<DownloadItem>) -> Result<()> {
    c.execute(send_chan).await
}

#[async_trait::async_trait]
pub trait Collector {
    async fn execute(&self, send_chan: Sender<DownloadItem>) -> Result<()>;
}
