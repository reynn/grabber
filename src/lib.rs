#[macro_use]
extern crate log;

use rawr::{prelude::*, structures::submission::Submission};
use std::{error::Error, path::Path};

pub mod config;
mod downloader;
mod errors;
mod filters;
mod user_lock;

use downloader::Downloadable;
use user_lock::UserLock;

pub fn start(config: config::AppConfig) -> Result<(), Box<dyn Error>> {
    let auth = if config.password.is_empty() {
        AnonymousAuthenticator::new()
    } else {
        PasswordAuthenticator::new(
            config.client_id.as_str(),
            config.client_secret.as_str(),
            config.username.as_str(),
            config.password.as_str(),
        )
    };

    let client = RedditClient::new("reddit-grabber(rust)", auth)?;

    let base_out_path = Path::new(config.output_path.as_str());

    info!(
        "[Configuration (output_path)]: {}",
        &base_out_path.display()
    );
    info!("[Configuration (client_id)]: {}", &config.client_id);
    info!("[Configuration (username)]: {}", &config.username);

    if let Some(users) = &config.users {
        for user in users.iter() {
            let user = client.user(user);
            let mut user_lock = UserLock::get(&config, user.name.as_str());
            info!("[Search] user {}", user.name);
            let list_opts = user_lock.get_list_opts();
            match user.submissions(list_opts) {
                Ok(listings) => {
                    let listings = listings
                        .filter(filters::filter_domains)
                        .map(Submission::from)
                        .collect::<Vec<_>>();

                    if listings.is_empty() {
                        continue;
                    };

                    user_lock.last_update_name = Some(listings[0].name().into());
                    user_lock.save()?;

                    for list_item in listings.into_iter() {
                        let mut d = Downloadable::from(&list_item);
                        d.user = user.name.clone();
                        if let Err(e) = d.download(&base_out_path) {
                            error!("Failed to download file: {:?}", e);
                        };
                    }
                }
                Err(e) => {
                    error!(
                        "[Error] while getting data for user ({}) [{}]",
                        user.name, e
                    );
                }
            };
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {}
