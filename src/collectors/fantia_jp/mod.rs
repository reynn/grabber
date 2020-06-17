mod fanclub;

use crate::{
    collectors::{errors::*, Collector},
    download::item::Item,
    config::AppConfig,
};
// use rayon::prelude::*;
use crossbeam_channel::Sender;
use async_trait::async_trait;

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

#[async_trait]
impl<'a> Collector for FantiaCollector<'a> {
    async fn execute(&self, _send_chan: Sender<Item>) -> Result<()> {
        let fanclubs = self.config.fantia.fan_clubs.clone();
        for fanclub_id in fanclubs {
            info!("Collecting items for Fantia fanclub {}", fanclub_id);
            let fanclub = self
                .client
                .get(format!("https://fantia.jp/api/v1/fanclubs/{}", fanclub_id).as_str())
                .header(reqwest::header::ACCEPT, "application/json")
                .header(
                    reqwest::header::COOKIE,
                    format!("_session_id={}", &self.config.fantia.session_id),
                )
                .send()
                .await?
                .json::<fanclub::FanClub>()
                .await?;
            fanclub
                .fanclub
                .recent_posts
                .iter()
                .for_each(|post| info!("Post: {:#?}", post.title));
        }
        Ok(())
    }
    fn get_name(&self) -> String {
        String::from("Fantia Collector")
    }
}
