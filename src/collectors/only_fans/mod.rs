use crate::{collectors::Collect, download::item::Item, config::AppConfig};
use crossbeam_channel::Sender;
use async_trait::async_trait;
use anyhow::Result;

#[derive(Debug)]
pub struct OnlyFansCollector {
    client: reqwest::Client,
    config: AppConfig,
}

impl OnlyFansCollector {
    pub fn new(config: AppConfig) -> Result<Self> {
        Ok(Self {
            client: reqwest::Client::new(),
            config,
        })
    }
}

#[async_trait]
impl Collect for OnlyFansCollector {
    async fn collect(&self, _send_chan: Sender<Item>) -> Result<()> {
        for of_user in self.config.only_fans.users.iter() {
            info!("Collecting items for OnlyFans user {}", of_user);
        }
        Ok(())
    }
    fn get_name(&self) -> String {
        String::from("Only Fans Collector")
    }
    fn is_enabled(&self, conf: &AppConfig) -> bool {
        conf.only_fans.enabled
    }
}
