use serde_derive::{Deserialize, Serialize};
use std::{fs::read_to_string, process::exit};

error_chain! {
    errors {
        IgnoredUser(u: String) {
            description("User is being ignored")
            display("User [{}] is being ignored", u)
        }
    }
    foreign_links {
        Io(std::io::Error);
        TOMLSerializeError(toml::ser::Error);
        TOMLDeserializeError(toml::de::Error);
    }
}

#[serde(default)]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppConfig {
    pub output_path: String,
    pub reddit: Reddit,
    pub only_fans: OnlyFans,
}

impl AppConfig {
    pub fn new(path: &str) -> Result<Self> {
        Ok(read_to_string(path)?.as_str().into())
    }
}

#[serde(default)]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct OnlyFans {
    pub users: Vec<String>,
}

impl From<&str> for OnlyFans {
    fn from(contents: &str) -> Self {
        toml::from_str(contents).unwrap_or_else(|err| {
            error!("Failed to create only_fans config from file [{}]", err);
            exit(2)
        })
    }
}

#[serde(default)]
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Reddit {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
    pub users: Vec<String>,
    pub friend_manage: bool,
}

impl Reddit {
    pub fn is_anonymous(&self) -> bool {
        self.username.is_empty() && self.password.is_empty()
    }
}

impl std::fmt::Debug for Reddit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_anonymous() {
            write!(f, "AppConfig(Anonymous)")
        } else {
            write!(
                f,
                "AppConfig(ClientID: {}, Username: {})",
                self.client_id, self.username
            )
        }
    }
}

impl From<&str> for Reddit {
    fn from(contents: &str) -> Self {
        toml::from_str(contents).unwrap_or_else(|err| {
            error!("Failed to create app config from file [{}]", err);
            exit(2)
        })
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
