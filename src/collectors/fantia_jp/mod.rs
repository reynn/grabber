mod fanclub;

use self::fanclub::Fanclub;
use crate::{collectors::Collect, download::item::Item, config::AppConfig};

use scraper::{Html, Selector};
use reqwest::header::HeaderMap;
use crossbeam_channel::Sender;
use async_trait::async_trait;
use anyhow::Result;
use thiserror::Error;

#[derive(Debug)]
pub struct FantiaCollector {
    client: reqwest::Client,
    config: AppConfig,
}

#[async_trait]
impl Collect for FantiaCollector {
    async fn collect(&self, _send_chan: Sender<Item>) -> Result<()> {
        let fanclubs = self.config.fantia.fan_clubs.clone();
        for fanclub_id in fanclubs {
            let fanclub = self.get_fanclub(fanclub_id.as_str()).await?;
            let posts = self.get_club_posts(&fanclub).await?;
            info!("Posts: {:?}", posts)
        }
        Ok(())
    }
    fn get_name(&self) -> String {
        String::from("Fantia Collector")
    }
    fn is_enabled(&self, conf: &AppConfig) -> bool {
        conf.fantia.enabled
    }
}

impl FantiaCollector {
    pub fn new(config: AppConfig) -> Result<Self> {
        Ok(Self {
            client: reqwest::Client::new(),
            config,
        })
    }

    fn get_headers(&self) -> Result<HeaderMap> {
        use reqwest::header::{ACCEPT, COOKIE};

        let mut map = HeaderMap::new();
        map.insert(ACCEPT, "application/json".parse()?);
        map.insert(
            COOKIE,
            format!("_session_id={}", &self.config.fantia.session_id).parse()?,
        );

        Ok(map)
    }

    pub async fn get_fanclub(&self, fanclub_id: &str) -> Result<Fanclub> {
        let result = self
            .client
            .get(format!("https://fantia.jp/api/v1/fanclubs/{}", fanclub_id).as_str())
            .headers(self.get_headers()?)
            .send()
            .await?
            .json::<fanclub::ClubContainer>()
            .await?;

        Ok(result.fanclub)
    }

    pub async fn get_club_posts(&self, fanclub: &Fanclub) -> Result<Vec<i32>> {
        let post_selector = Selector::parse(POST_SELECTOR_PATH)
            .map_err(|e| FantiaErrors::SelectorParseError(format!("{:?}", e).into()))?;
        info!("PostSelector: {:?}", post_selector);
        info!("Collecting items for Fantia fanclub {}", fanclub.user.name);
        let client = self
            .client
            .get(format!("https://fantia.jp/fanclubs/{}/posts?page={}", fanclub.id, 1).as_str())
            .headers(self.get_headers()?);
        let response = &client.send().await?.text().await?;
        let doc = Html::parse_document(response.as_str());

        let selected = doc.select(&post_selector).next();
        info!("Selected: {:?}", selected);

        Ok(Vec::new())
    }
}

#[derive(Error, Debug)]
pub enum FantiaErrors {
    #[error("There was an error")]
    SelectorParseError(String),
}

static POST_SELECTOR_PATH: &str = "div.masonry-item";
