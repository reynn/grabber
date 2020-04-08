use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use rawr::prelude::*;
use rawr::structures::submission::Submission;
use std::error::Error;
use std::path::Path;

pub mod config;
pub mod errors;
pub mod user_lock;

#[allow(dead_code)]
struct Downloadable {
    url: String,
    domain: String,
    title: String,
}

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

    println!("[Configuration (output_path)]: {}", base_out_path.display());
    println!("[Configuration (client_id)]: {}", config.client_id);
    println!("[Configuration (username)]: {}", config.username);

    // let mut downloads: Vec<Downloadable> = Vec::new();

    if let Some(users) = &config.users {
        for user in users.iter() {
            let user = client.user(user);
            let mut user_lock = user_lock::UserLock::get(&config, user.name.as_str());
            println!("{}", &user_lock);
            println!("Searching for user {}", user.name);
            let mut submissions: Vec<Submission> = Vec::new();
            let list_opts = user_lock.get_list_opts();
            match user.submissions(list_opts) {
                Ok(list) => {
                    for submission in list.into_iter() {
                        submissions.push(submission)
                    }
                }
                Err(e) => {
                    eprintln!("Error while getting data for user ({}) [{}]", user.name, e);
                }
            };
            println!(
                "submissions: \n{:#?}",
                submissions
                    .iter()
                    .map(|s| {
                        let dt = DateTime::<Utc>::from_utc(
                            NaiveDateTime::from_timestamp(s.created_utc(), 0),
                            Utc,
                        );
                        format!(
                            "[{}] ({}:{}) {}",
                            s.author().name,
                            s.name(),
                            dt.to_rfc3339(),
                            s.title()
                        )
                    })
                    .into_iter()
                    .collect::<Vec<String>>()
            );
            &user_lock.save();
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}

