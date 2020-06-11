use super::errors::*;

use crate::{collectors::Collector, config::AppConfig};

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
    async fn execute(&self) -> Result<()> {
        info!("Collecting items from Only Fans");
        Ok(())
    }
}
