mod errors;
mod fantia;
mod only_fans;
mod reddit;

use self::errors::*;
use crate::collectors::Collector;

use serde_derive::{Deserialize, Serialize};
use std::{fs::read_to_string, process::exit};

#[serde(default)]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct General {
    pub output_path: String,
    pub collector_subfolders: bool,
}

#[serde(default)]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppConfig {
    pub general: General,
    pub fantia: fantia::Fantia,
    pub only_fans: only_fans::OnlyFans,
    pub reddit: reddit::Reddit,
}

impl AppConfig {
    pub fn new(path: &str) -> Result<Self> {
        info!("Loading config from [{}]", path);
        Ok(read_to_string(path)?.as_str().into())
    }

    pub fn get_collector_outpath(&self, collector: impl Collector) -> Result<String> {
        Ok(collector.get_name())
    }
}

impl From<&str> for AppConfig {
    fn from(contents: &str) -> Self {
        toml::from_str(contents).unwrap_or_else(|err| {
            error!("Failed to create app config from file [{}]", err);
            exit(2)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toml_parse() {
        let contents = r#"

            [reddit]
            client_id = "the-base-client"
            client_secret = "super-secret"
            username = "test-user"
            password = "hello-password"
        "#;
        let conf = AppConfig::from(contents);
        let uname = conf.reddit.username;
        assert_eq!(uname, "test-user")
    }
}
