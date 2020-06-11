use crate::{
    collectors::{errors::*, Collector},
    download::DownloadItem,
    config::AppConfig,
};
use crossbeam_channel::Sender;

#[derive(Debug)]
pub struct OnlyFansCollector<'a> {
    client: reqwest::Client,
    config: &'a AppConfig,
}

impl<'a> OnlyFansCollector<'a> {
    pub fn new(config: &'a AppConfig) -> Result<Self> {
        Ok(Self {
            client: reqwest::Client::new(),
            config,
        })
    }
}

#[async_trait::async_trait]
impl<'a> Collector for OnlyFansCollector<'a> {
    async fn execute(&self, _send_chan: Sender<DownloadItem>) -> Result<()> {
        info!("Collecting items from Only Fans");
        for of_user in self.config.only_fans.users.iter() {
          info!("Collecting data for {}", of_user);
        }
        Ok(())
    }
}
