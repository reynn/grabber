#[macro_use]
extern crate log;

pub mod config;
mod download;
mod errors;
mod filters;
mod user_lock;

use std::{error::Error, path::Path};

use rawr::{prelude::*, structures::submission::Submission};

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

    info!("[Config (output_path)]: {}", &base_out_path.display());

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
                        if let Err(d_err) = d.download(&base_out_path) {
                            error!("Failed to download: {:?}", d_err)
                        }
                    }
                }
                Err(e) => {
                    error!("failed to get data for user [{}] error({})", user.name, e);
                }
            };
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {}
