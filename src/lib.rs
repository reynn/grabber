use rawr::prelude::*;
use rawr::structures::submission::Submission;
use std::error::Error;
use std::path::Path;

pub mod config;
pub mod downloader;
pub mod errors;
pub mod user_lock;
pub mod filters;

use downloader::Downloadable;
use user_lock::UserLock;

pub fn start(config: config::AppConfig) -> Result<(), Box<dyn Error>> {
    let client = RedditClient::new(
        "reddit-grabber(rust)",
        PasswordAuthenticator::new(
            config.client_id.as_str(),
            config.client_secret.as_str(),
            config.username.as_str(),
            config.password.as_str(),
        ),
    )?;

    let base_out_path = Path::new(config.output_path.as_str());

    println!(
        "[Configuration (output_path)]: {}",
        &base_out_path.display()
    );
    println!("[Configuration (client_id)]: {}", &config.client_id);
    println!("[Configuration (username)]: {}", &config.username);

    if let Some(users) = &config.users {
        for user in users.iter() {
            let user = client.user(user);
            let mut user_lock = UserLock::get(&config, user.name.as_str());
            println!("{}", &user_lock);
            println!("[Search] user {}", user.name);
            let list_opts = user_lock.get_list_opts();
            match user.submissions(list_opts) {
                Ok(listings) => {
                    let listings = listings
                        .filter(filters::filter_domains)
                        .map(|l| Submission::from(l))
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
                            eprintln!("Failed to download file: {:?}", e);
                        };
                    }
                }
                Err(e) => {
                    eprintln!(
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
