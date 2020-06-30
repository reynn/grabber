mod fanclub;
mod post;

use self::fanclub::FanclubInner;
use crate::{collectors::Collect, download::item::Item, config::AppConfig};

use scraper::{Html, Selector};
use reqwest::header::HeaderMap;
use async_channel::Sender;
use async_trait::async_trait;
use anyhow::Result;
use thiserror::Error;

#[derive(Debug)]
pub struct Collector {
    client: reqwest::Client,
    config: AppConfig,
}

#[async_trait]
impl Collect for Collector {
    async fn collect(&self, send_chan: Sender<Item>) -> Result<()> {
        let fanclubs = self.config.fantia.fan_clubs.clone();
        for fanclub_id in fanclubs {
            match self.get_fanclub(fanclub_id).await {
                Ok(fanclub) => match self.get_club_post_ids(&fanclub, 1).await {
                    Ok(post_ids) => {
                        info!("Found {} posts", post_ids.len());
                        // info!("Posts :: {:#?}", post_ids);
                        for id in post_ids {
                            if let Err(download_err) =
                                self.get_post_downloadables(&fanclub, id.as_str(), &send_chan).await
                            {
                                error!("Failed to get downloads for post {} [{}]", id, download_err);
                            }
                        }
                    }
                    Err(club_posts_err) => error!(
                        "Failed to gather data for {} [{}]",
                        fanclub.creator_name, club_posts_err
                    ),
                },
                Err(fanclub_err) => error!("Failed to get the fanclub information {}", fanclub_err),
            }
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

impl Collector {
    pub fn new(config: AppConfig) -> Result<Self> {
        Ok(Self {
            client: reqwest::Client::new(),
            config,
        })
    }

    fn get_headers(&self) -> Result<HeaderMap> {
        use reqwest::header::{USER_AGENT, COOKIE};

        let mut map = HeaderMap::new();
        map.insert(
            USER_AGENT,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:77.0) Gecko/20100101 Firefox/77.0".parse()?,
        );
        map.insert(
            COOKIE,
            format!("_session_id={}", &self.config.fantia.session_id).parse()?,
        );

        Ok(map)
    }

    async fn get_fanclub<D: std::fmt::Display>(&self, fanclub_id: D) -> Result<FanclubInner> {
        let result = self
            .client
            .get(format!("{}/api/v1/fanclubs/{}", FANTIA_BASE_URL, fanclub_id).as_str())
            .headers(self.get_headers()?)
            .send()
            .await?
            .json::<fanclub::Fanclub>()
            .await?;

        Ok(result.inner)
    }

    async fn get_club_post_ids(&self, fanclub: &FanclubInner, start_page_num: i16) -> Result<Vec<String>> {
        let post_selector = Selector::parse(POST_SELECTOR_PATH)
            .map_err(|e| FantiaErrors::SelectorParseError(format!("{:?}", e).into()))?;

        let mut selections = Vec::new();
        let mut page_num = start_page_num;
        loop {
            info!("Finding posts for fanclub {} page {}", fanclub.user.name, page_num);
            let endpoint = format!("{}/fanclubs/{}/posts?page={}", FANTIA_BASE_URL, fanclub.id, page_num);
            debug!("Getting data from {}", endpoint);
            let req_builder = self.client.get(endpoint.as_str()).headers(self.get_headers()?);
            let response = req_builder.send().await?;
            let response_body: String = response.text().await?;

            let document = Html::parse_document(response_body.as_str());
            let elements = document.select(&post_selector);

            let mut post_links: Vec<String> = elements.filter_map(|selection| get_post_id(selection)).collect();
            if !post_links.is_empty() {
                selections.append(&mut post_links);
            } else {
                break;
            }
            page_num += 1;
        }

        Ok(selections)
    }

    async fn get_post_downloadables(
        &self,
        fanclub: &FanclubInner,
        post_id: &'_ str,
        send_chan: &Sender<Item>,
    ) -> Result<()> {
        info!(
            "Discovering items to download from the {} club post {}",
            &fanclub.name, post_id
        );
        let result = self
            .client
            .get(format!("{}/api/v1/posts/{}", FANTIA_BASE_URL, post_id).as_str())
            .headers(self.get_headers()?)
            .send()
            .await?
            .json::<post::Post>()
            .await?;

        for content in result.inner.post_contents.into_iter() {
            // info!("Post Content :: {:#?}", content);
            if let Some(uri) = content.download_uri {
                let url = reqwest::Url::parse(format!("{}{}", FANTIA_BASE_URL, uri).as_str())?;
                info!("Downloadable URL :: {}", url.as_str());
                send_chan
                    .send(Item::new(
                        url,
                        self.get_name(),
                        Some(String::from(&fanclub.creator_name)),
                    ))
                    .await?;
            }
            if let Some(photos) = content.post_content_photos {
                for photo in photos {
                    // Any of these could be empty, we'll try to do some fallbacks
                    let mut url = &photo.url.original;
                    if url == "" {
                        url = &photo.url.large;
                    }
                    if url == "" {
                        url = &photo.url.medium;
                    }
                    if let Ok(url) = reqwest::Url::parse(url.as_str()) {
                        info!("Downloadable URL :: {}", &url);
                        if let Err(send_err) = send_chan
                            .send(Item::new(
                                url.clone(),
                                self.get_name(),
                                Some(String::from(&fanclub.creator_name)),
                            ))
                            .await
                        {
                            error!("Failed to send item[{}] to the download manager {}", url, send_err);
                        };
                    }
                }
            }
        }

        Ok(())
    }
}

fn get_post_id(selection: scraper::ElementRef<'_>) -> Option<String> {
    if let Some(href) = selection.value().attr("href") {
        let id = href.trim_start_matches("/posts/");
        Some(id.into())
    } else {
        None
    }
}

#[derive(Error, Debug)]
pub enum FantiaErrors {
    #[error("There was an error with the provided selector [{0}]")]
    SelectorParseError(String),
}

static FANTIA_BASE_URL: &str = r#"https://fantia.jp"#;
static POST_SELECTOR_PATH: &str = r#"a[class="link-block"]"#;
