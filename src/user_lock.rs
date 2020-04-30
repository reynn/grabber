use crate::config;
use chrono::Utc;
use rawr::options::{ListingAnchor, ListingOptions};
use serde_derive::{Deserialize, Serialize};
use std::path::Path;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        TOMLSerializeError(toml::ser::Error);
        TOMLDeserializeError(toml::de::Error);
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct UserLock {
    pub file_name: String,
    pub name: Option<String>,
    pub last_update_name: Option<String>,
    #[serde(default)]
    ignore: bool,
    #[serde(skip)]
    pub last_checked: Option<chrono::DateTime<Utc>>,
}

impl std::fmt::Display for UserLock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.ignore {
            write!(
                f,
                "User {} is being ignored due to failures",
                &self.name.clone().unwrap_or_else(|| "none".into())
            )
        } else {
            write!(
                f,
                "User {} last updated, {}, at post id {}.",
                self.name.clone().unwrap_or_else(|| "none".into()),
                self.last_checked
                    .clone()
                    .unwrap_or_else(Utc::now)
                    .format("%Y-%m-%d (%H:%M:%S)"),
                self.last_update_name.clone().unwrap_or_else(|| "".into()),
            )
        }
    }
}

impl From<&str> for UserLock {
    fn from(content: &str) -> Self {
        toml::from_str(content).unwrap()
    }
}

impl UserLock {
    fn new(username: String, lock_file_name: String) -> Self {
        Self {
            file_name: lock_file_name,
            name: Some(username),
            ignore: false,
            last_update_name: None,
            last_checked: None,
        }
    }

    /// Gets a user lock file based on provided username and app config
    ///
    /// ## Examples
    ///
    /// ```rust
    ///
    /// ```
    pub fn get(config: &config::AppConfig, username: &str) -> Self {
        let user_lock_base = Path::new(&config.output_path).join(username);
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
                    )
                }
            },
            Err(_) => {
                warn!(
                    "Couldn't find the user lock file for {}, creating initial...",
                    username
                );
                let mut this = Self::new(
                    username.to_string(),
                    String::from(user_lock_file.to_str().unwrap_or_else(|| "")),
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
