use serde_derive::{Deserialize, Serialize};
use std::error::Error;

#[derive(Default, Deserialize, Serialize)]
pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
    pub output_path: String,
    pub users: Option<Vec<String>>,
}

impl AppConfig {
    pub fn new(file_name: &str) -> Result<Self, &dyn Error> {
        let conf: Self = std::fs::read_to_string(file_name)
            .expect("Failed to open file")
            .as_str()
            .into();
        Ok(conf)
    }
}

impl std::fmt::Debug for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let client_id = &self.client_id;
        let username = &self.username;
        write!(
            f,
            "AppConfig(ClientID: {}, Username: {})",
            client_id, username
        )
    }
}

impl From<&str> for AppConfig {
    fn from(contents: &str) -> Self {
        toml::from_str(contents).expect("Failed to create config from provided file")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toml_parse() {
        let contents = r#"
            client_id = "the-base-client"
            client_secret = "super-secret"
            username = "test-user"
            password = "hello-password"
        "#;
        let conf = AppConfig::from(contents);
        assert_eq!(conf.username, "test-user")
    }
}
