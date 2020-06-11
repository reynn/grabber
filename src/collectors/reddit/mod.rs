pub mod user_lock;

use crate::{
    collectors::{errors::*, Collector, reddit::user_lock::UserLock},
    download::DownloadItem,
    config::AppConfig,
};
use crossbeam_channel::Sender;
use rawr::prelude::*;

pub struct RedditCollector<'a> {
    reddit_client: RedditClient,
    config: &'a AppConfig,
}

impl<'a> RedditCollector<'a> {
    pub fn new(config: &'a AppConfig) -> Result<Self> {
        let auth = if config.reddit.is_anonymous() {
            debug!("Creating an anonymous authenticator");
            AnonymousAuthenticator::new()
        } else {
            debug!(
                "Creating a password authenticator [client({}), username({})]",
                &config.reddit.client_id, &config.reddit.username
            );
            PasswordAuthenticator::new(
                &config.reddit.client_id,
                &config.reddit.client_secret,
                &config.reddit.username,
                &config.reddit.password,
            )
        };

        let client = RedditClient::new("grabber-rs-reddit", auth)?;

        Ok(Self {
            config,
            reddit_client: client,
        })
    }
}

#[async_trait::async_trait]
impl<'a> Collector for RedditCollector<'a> {
    async fn execute(&self, send_chan: Sender<DownloadItem>) -> Result<()> {
        info!("Collecting Reddit user posts");
        let mut user_list = self.config.reddit.users.clone();

        user_list.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        user_list.dedup();

        for user_name in user_list.iter() {
            info!("Collecting posts for Reddit user {}", user_name);
            let reddit_user = self.reddit_client.user(user_name);
            let mut lock_file = UserLock::get(&self.config, user_name);
            if lock_file.is_ignored() {
                warn!("Skipping {} due to ignore", user_name);
                continue;
            }
            if let Ok(submissions) = reddit_user.submissions(lock_file.get_list_opts()) {
                for submission in submissions.take(10) {
                    info!("{}: {}", user_name, submission.title());
                    let mut download_item: DownloadItem = submission.into();
                    download_item.sub_path = Some(user_name.into());
                    send_chan.send(download_item)?;
                }
            } else {
                lock_file.ignore()?;
            };
        }

        Ok(())
    }
}
