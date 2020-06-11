//! Collectors gather data from sites
//!

pub mod errors;
pub mod fantia_jp;
pub mod only_fans;
pub mod reddit;

use errors::*;

pub async fn run_collector(c: impl Collector) -> Result<()> {
    c.execute().await
}

#[async_trait::async_trait]
pub trait Collector {
    async fn execute(&self) -> Result<()>;
}
