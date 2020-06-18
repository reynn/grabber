pub mod listing;
pub mod user;

use super::{Collect, user_lock::UserLock};
use crate::{download::item::Item, config::AppConfig};

use async_trait::async_trait;
use crossbeam_channel::Sender;
use anyhow::Result;

pub struct RedditCollector {
    client: reqwest::Client,
    config: AppConfig,
}

impl std::fmt::Debug for RedditCollector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RedditCollector(Users({}))", self.config.reddit.users.len())
    }
}

impl RedditCollector {
    pub fn new(config: AppConfig) -> Result<Self> {
        Ok(Self {
            config: config,
            client: reqwest::Client::new(),
        })
    }
}

#[async_trait]
impl Collect for RedditCollector {
    async fn collect(&self, _send_chan: Sender<Item>) -> Result<()> {
        info!("Collecting Reddit user posts");
        let mut user_list = self.config.reddit.users.clone();

        user_list.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        user_list.dedup();

        for user_name in user_list.iter() {
            info!("Collecting posts for Reddit user {}", user_name);
            debug!("{:#?}", self);
            let _reddit_response = self.client.get("").send().await?;
            // let reddit_user = self.client.user(user_name);
            let lock_file = UserLock::get(&self.config, user_name, self.get_name().as_str());
            if lock_file.is_ignored() {
                warn!("Skipping {} due to ignore", user_name);
                continue;
            }
            // if let Ok(submissions) = reddit_user.submissions(lock_file.get_list_opts()) {
            //     for submission in submissions.take(10) {
            //         info!("{}: {}", user_name, submission.title());
            //         let mut download_item: Item = submission.into();
            //         download_item.sub_path = Some(user_name.into());
            //         send_chan.send(download_item)?;
            //     }
            // } else {
            //     lock_file.ignore()?;
            // };
        }

        Ok(())
    }
    fn get_name(&self) -> String {
        String::from("Reddit Collector")
    }
    fn is_enabled(&self, conf: &AppConfig) -> bool {
        conf.reddit.enabled
    }
}
