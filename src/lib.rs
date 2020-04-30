#![warn(missing_debug_implementations, rust_2018_idioms)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;

pub mod config;
mod download;
mod filters;
mod user_lock;

use crate::{
    download::{DownloadItem, DownloadManager},
    user_lock::UserLock,
};

use std::cmp::Ordering;

use chrono::{DateTime, NaiveDateTime, Utc};
use rawr::prelude::*;
use rawr::structures::submission::Submission;
use rayon::prelude::*;

error_chain! {
    errors {
        IgnoredUser(u: String) {
            description("User is being ignored")
            display("User [{}] is being ignored", u)
        }
        NoNewPosts(u: String, lu: String) {
            description("User has no new posts")
            display("{} has no new posts. Last updated ", u)
        }
        // UserNoLongerExists(u: String, status: i32) {
        //     description("Received an HTTP error when attempting to access the user")
        //     display("HTTP {} occurred when trying to get data for user {}", status, u)
        // }
    }

    foreign_links {
        Io(std::io::Error);
        TOMLSerializeError(toml::ser::Error);
        TOMLDeserializeError(toml::de::Error);
        RawrError(rawr::errors::APIError);
        LockError(user_lock::Error);
    }
}

pub fn start(config: &mut config::AppConfig) -> Result<()> {
    let auth = if config.is_anonymous() {
        debug!("Creating an anonymous authenticator");
        AnonymousAuthenticator::new()
    } else {
        debug!(
            "Creating a password authenticator [client({}), username({})]",
            &config.client_id, &config.username
        );
        PasswordAuthenticator::new(
            &config.client_id,
            &config.client_secret,
            &config.username,
            &config.password,
        )
    };

    let client = RedditClient::new("grabber-rs", auth)?;

    let download_manager = DownloadManager::new(config.output_path.as_str());

    info!("Sorting and removing duplicate users before processing");
    let mut users = config.users.clone();
    debug!("Sort using case insensitive matching");
    users.sort_by(sort_string_insensitive);
    users.dedup();

    for (idx, user_chunks) in users.chunks(10).enumerate() {
        let _ = user_chunks
            .into_par_iter()
            .map(|user| {
                info!("User {}", user);
                let mut user_lock = UserLock::get(&config, user);
                match get_downloadable_user_posts(&client, &user_lock, user) {
                    Ok(update) => {
                        info!("User update {}", update);
                        download_manager.download_items(&update.downloads);
                        update_lock_file(&mut user_lock, Some(update))
                            .unwrap_or_else(|e| error!("Error while saving lock file {:?}", e));
                    }
                    Err(Error(ErrorKind::IgnoredUser(u), _)) => {
                        warn!("{} is being ignored", u);
                    }
                    Err(Error(ErrorKind::NoNewPosts(u, lu), _)) => {
                        warn!("{} has no new posts. last updated {}", u, lu);
                    }
                    Err(e) => {
                        error!("There was a general error [{}]", e);
                        user_lock.ignore().unwrap_or_else(|ie| {
                            error!("Attempted to ignore user but failed: {:?}", ie)
                        })
                    }
                }
            })
            .collect::<Vec<_>>();
        info!("Completed chunk #{}!", idx);
    }
    Ok(())
}

fn sort_string_insensitive(a: &String, b: &String) -> Ordering {
    a.to_lowercase().cmp(&b.to_lowercase())
}

fn update_lock_file(lock: &mut UserLock, user_update: Option<UserUpdate>) -> Result<()> {
    if let Some(update) = user_update {
        lock.last_update_name = Some(update.last_post_id.to_string());
        lock.last_checked = Some(update.last_updated);
    }
    Ok(lock.save()?)
}

#[derive(Debug)]
struct UserUpdate {
    last_updated: DateTime<Utc>,
    last_post_id: String,
    downloads: Vec<DownloadItem>,
}

impl std::fmt::Display for UserUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User has {} new downloads", self.downloads.len())
    }
}

impl Default for UserUpdate {
    fn default() -> Self {
        Self {
            last_updated: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            last_post_id: String::new(),
            downloads: Vec::new(),
        }
    }
}

fn get_downloadable_user_posts(
    client: &RedditClient,
    user_lock: &UserLock,
    user: &str,
) -> Result<UserUpdate> {
    ensure!(
        !&user_lock.is_ignored(),
        ErrorKind::IgnoredUser(user.into())
    );
    let last_checked: String = if let Some(update_stamp) = user_lock.last_checked {
        update_stamp.to_rfc3339()
    } else {
        "Never".into()
    };
    let user = client.user(user);
    let list_opts = user_lock.get_list_opts();
    match user.submissions(list_opts) {
        Ok(user_listings) => {
            let user_listings: Vec<Submission<'_>> = user_listings.collect();

            ensure!(
                !user_listings.is_empty(),
                ErrorKind::NoNewPosts(user.name, last_checked)
            );

            let mut user_update = UserUpdate::default();

            if let Some(latest_post) = user_listings.first() {
                user_update.last_post_id = latest_post.name().into();
                user_update.last_updated = DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(latest_post.created_utc(), 0),
                    Utc,
                );
            }

            user_update.downloads = user_listings
                .iter()
                .filter_map(|listing| {
                    if let Some(handleable) = filters::filter_domains(&listing) {
                        Some(DownloadItem::new(
                            handleable,
                            listing.name().to_string(),
                            user.name.to_owned(),
                        ))
                    } else {
                        None
                    }
                })
                .collect();

            Ok(user_update)
        }
        Err(e) => Err(ErrorKind::RawrError(e).into()),
    }
}

#[cfg(test)]
mod tests {}
