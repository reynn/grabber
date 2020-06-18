use std::path::Path;

use crate::{config::AppConfig, collectors::reddit::listing};

use serde_derive::{Deserialize, Serialize};
use chrono::Utc;
use anyhow::Result;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct UserLock {
    pub file_name: String,
    pub name: Option<String>,
    pub last_update_name: Option<String>,
    pub last_post_id: Option<String>,
    pub collector_name: Option<String>,
    #[serde(default)]
    ignore: bool,
    #[serde(skip)]
    pub last_checked: Option<chrono::DateTime<Utc>>,
}

impl UserLock {
    fn new(username: String, lock_file_name: String, collector_name: &str) -> Self {
        Self {
            file_name: lock_file_name,
            name: Some(username),
            ignore: false,
            last_update_name: None,
            last_checked: None,
            last_post_id: None,
            collector_name: Some(String::from(collector_name)),
        }
    }
    pub fn get(config: &AppConfig, username: &str, collector_name: &str) -> Self {
        // If the user has provided a reddit specific outpath we want to use that instead of the default
        let mut user_lock_base = if let Some(reddit_output) = &config.reddit.output_path {
            Path::new(reddit_output.as_str()).to_path_buf()
        } else {
            let mut out_path = Path::new(&config.general.output_path).to_path_buf();
            // adding the collector subfolder should only be necessary if there isn't a specific path
            if config.general.collector_subfolders {
                out_path = out_path.join("reddit")
            }
            out_path
        };
        user_lock_base = user_lock_base.join(username);
        let user_lock_file = user_lock_base.join(".user_lock");
        if !user_lock_base.exists() {
            debug!("Creating directory ({})", user_lock_base.display());
            std::fs::create_dir_all(user_lock_file.parent().unwrap()).unwrap();
        }

        // try to read the user_lock file
        // If successful we use toml parser to load the values into our struct
        // if not we acknowledge the error and provide a default struct with just a username
        match std::fs::read_to_string(user_lock_file.as_path()) {
            Ok(contents) => match toml::from_str(contents.as_str()) {
                Ok(res) => res,
                Err(e) => {
                    error!("Failed to parse user lock file {:?}", e);
                    Self::new(
                        username.to_string(),
                        String::from(user_lock_file.to_str().unwrap_or_else(|| "")),
                        collector_name,
                    )
                }
            },
            Err(_) => {
                warn!("Couldn't find the user lock file for {}, creating initial...", username);
                let mut this = Self::new(
                    username.to_string(),
                    String::from(user_lock_file.to_str().unwrap_or_else(|| "")),
                    collector_name,
                );

                if let Err(save_error) = this.save() {
                    error!("Failed to save the initial user_lock {}", save_error);
                };

                this
            }
        }
    }

    pub fn ignore(&mut self) -> Result<()> {
        self.ignore = true;
        self.save()
    }

    pub fn is_ignored(&self) -> bool {
        self.ignore
    }

    pub fn save(&mut self) -> Result<()> {
        self.last_checked = Some(Utc::now());

        std::fs::write(&self.file_name, toml::to_string(&self)?)?;

        Ok(())
    }

    pub fn get_list_opts(&self) -> listing::Options {
        if let Some(name) = &self.last_update_name {
            listing::Options {
                batch_size: 20,
                anchor: Some(listing::Anchor::Before(name.into())),
            }
        } else {
            listing::Options::default()
        }
    }
}
