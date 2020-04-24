#![warn(missing_debug_implementations, rust_2018_idioms)]
#[macro_use]
extern crate log;

pub mod config;
mod download;
mod errors;
mod filters;
mod user_lock;

use std::{error::Error, path::Path};

use rawr::prelude::*;
use rayon::prelude::*;

use crate::{download::Downloadable, user_lock::UserLock};

pub fn start(config: config::AppConfig) -> Result<(), Box<dyn Error>> {
    let auth = if config.is_anonymous() {
        debug!("Creating an anonymous authenticator");
        AnonymousAuthenticator::new()
    } else {
        debug!(
            "Creating a password authenticator [client({}), username({})]",
            &config.client_id(),
            &config.username()
        );
        PasswordAuthenticator::new(
            &config.client_id(),
            &config.client_secret(),
            &config.username(),
            &config.password(),
        )
    };

    let client = RedditClient::new("grabber-rs", auth)?;

    let base_out_path = Path::new(config.output_path.as_str());

    info!("(output_path): {}", &base_out_path.display());

    if let Some(users) = &config.users {
        let _ = users
            .into_iter()
            .map(|x| handle_user(&config, &client, &base_out_path, x))
            .collect::<Vec<_>>();
    }

    Ok(())
}

fn handle_user(
    config: &config::AppConfig,
    client: &RedditClient,
    base_out_path: &&Path,
    user: &String,
) {
    let user = client.user(user);
    let mut user_lock = UserLock::get(&config, &user.name.as_str());
    if user_lock.is_ignored() {
        warn!("{}", user_lock);
        return;
    }
    info!("{}", user_lock);
    let list_opts = user_lock.get_list_opts();
    match user.submissions(list_opts) {
        Ok(user_listings) => {
            let user_downloads: Vec<Downloadable> = user_listings
                .filter_map(|listing| match filters::filter_domains(&listing) {
                    None => None,
                    Some(handleable) => Some(Downloadable::new(
                        handleable,
                        listing.name().to_string(),
                        listing.title().to_string(),
                        user.name.to_string().to_owned(),
                    )),
                })
                .collect();

            if !user_downloads.is_empty() {
                user_lock.last_update_name =
                    Some(user_downloads.get(0).expect("nope").post_id.to_owned());
                user_lock.save().unwrap_or_else(|e| {
                    error!("Failed to save User Lock file [{}]", e);
                    return;
                });

                for d in user_downloads.into_iter() {
                    debug!("Downloading {:?}", d);
                    if let Err(d_err) = d.download(&base_out_path) {
                        error!("Failed to download: {:?}", d_err)
                    }
                }
            };
        }
        Err(e) => {
            if let Err(e) = user_lock.ignore() {
                error!(
                    "Failed to set user {} as ignored {}",
                    user.name,
                    e.to_string()
                );
            }
            error!(
                "failed to get data for user [{}] error({})",
                user.name,
                e.to_string()
            );
        }
    };
}

#[cfg(test)]
mod tests {}
