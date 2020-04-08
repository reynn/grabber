use crate::config;
use rawr::options::{ListingAnchor, ListingOptions};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLock {
    pub file_name: String,
    pub name: Option<String>,
    pub last_update_name: Option<String>,
    pub timestamp: Option<SystemTime>,
}

impl std::fmt::Display for UserLock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[User Lock ({})] last update [{:#?}] full_name {}.",
            self.name.clone().unwrap_or("none".into()),
            self.timestamp.clone().unwrap_or(SystemTime::now()),
            self.last_update_name.clone().unwrap_or("".into()),
        )
    }
}

impl From<&str> for UserLock {
    fn from(content: &str) -> Self {
        toml::from_str(content).unwrap()
    }
}

impl UserLock {
    pub fn get(config: &config::AppConfig, username: &str) -> Self {
        let user_lock_file = Path::new(config.borrow().output_path.as_str())
            .join(username)
            .join(".user_lock");
        if !user_lock_file.as_path().exists() {
            println!("[User Lock ({})] file does not exist", username);
            std::fs::create_dir_all(user_lock_file.parent().unwrap()).unwrap();
        }
        println!(
            "[User Lock ({})] filename: {}",
            username,
            user_lock_file.display()
        );

        // try to read the user_lock file
        // If successful we use toml parser to load the values into our struct
        // if not we acknowledge the error and provide a default struct with just a username
        match std::fs::read_to_string(user_lock_file.as_path()) {
            Ok(contents) => match toml::from_str(contents.as_str()) {
                Ok(res) => res,
                Err(_) => Self {
                    file_name: String::from(user_lock_file.to_str().unwrap()),
                    name: Some(String::from(username)),
                    last_update_name: None,
                    timestamp: None,
                },
            },
            Err(_) => {
                let mut this = Self {
                    file_name: String::from(user_lock_file.to_str().unwrap()),
                    name: Some(String::from(username)),
                    last_update_name: None,
                    timestamp: None,
                };

                if let Err(save_error) = this.save() {
                    eprintln!("failed to save the initial user_lock {}", save_error);
                }

                this
            }
        }
    }

    pub fn save(&mut self) -> Result<(), std::io::Error> {
        self.timestamp = Some(SystemTime::now());
        let toml_out = toml::to_string(&self).unwrap_or_else(|err| {
            println!("Failed to serialize to TOML [{}]", err);
            String::new()
        });

        std::fs::write(&self.file_name, toml_out)
    }

    pub fn get_list_opts(&self) -> ListingOptions {
        if let Some(name) = &self.last_update_name {
            ListingOptions {
                batch: 20,
                anchor: ListingAnchor::Before(name.into()),
            }
        } else {
            ListingOptions::default()
        }
    }
}

