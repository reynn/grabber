//! Collectors gather data from sites
//!

pub mod fantia_jp;
pub mod only_fans;
pub mod reddit;
pub mod user_lock;

use crossbeam_channel::Sender;
use async_trait::async_trait;
use anyhow::Result;

use crate::{download::item::Item, config::AppConfig};

#[derive(Default, Debug)]
pub struct Collectors {
    pub collectors: Vec<Box<dyn Collect>>,
}

impl Collectors {
    pub async fn run(&self, config: &AppConfig, send_chan: Sender<Item>) -> Result<()> {
        let mut running_collectors = Vec::new();
        for collector in self.collectors.iter() {
            if collector.is_enabled(config) {
                info!("Collector {} is starting", collector.get_name());
                let running_collector = collector.collect(send_chan.clone());
                running_collectors.push(running_collector);
                collector.collect(send_chan.clone()).await?;
            } else {
                info!("Collector {} is not enabled", collector.get_name())
            }
        }
        Ok(())
    }
}

#[async_trait]
pub trait Collect {
    async fn collect(&self, send_chan: Sender<Item>) -> Result<()>;
    fn is_enabled(&self, conf: &AppConfig) -> bool;
    fn get_name(&self) -> String;
}

impl std::fmt::Debug for dyn Collect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Collector: {}", self.get_name())
    }
}
