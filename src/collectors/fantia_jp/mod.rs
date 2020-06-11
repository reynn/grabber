mod fanclub;

use crate::{
    collectors::{errors::*, Collector},
    download::DownloadItem,
    config::AppConfig,
};
use crossbeam_channel::Sender;

#[derive(Debug)]
pub struct FantiaCollector<'a> {
    client: reqwest::Client,
    config: &'a AppConfig,
}

impl<'a> FantiaCollector<'a> {
    pub fn new(config: &'a AppConfig) -> Result<Self> {
        Ok(Self {
            client: reqwest::Client::new(),
            config,
        })
    }
}

#[async_trait::async_trait]
impl<'a> Collector for FantiaCollector<'a> {
    async fn execute(&self, _send_chan: Sender<DownloadItem>) -> Result<()> {
        info!("Collecting items from Fantia");
        for of_user in self.config.fantia.fan_clubs.iter() {
          info!("Collecting data for {}", of_user);
        }
        Ok(())
    }
}
