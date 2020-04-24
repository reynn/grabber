use serde_derive::{Deserialize, Serialize};

macro_rules! getter {
    ($var:ident, $ret:ty) => {
        pub fn $var(&self) -> $ret {
            if self.$var.is_some() {
                self.$var.clone().unwrap()
            } else {
                "".into()
            }
        }
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct AppConfig {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub output_path: String,
    pub users: Option<Vec<String>>,
}

impl AppConfig {
    getter!(client_id, String);
    getter!(client_secret, String);
    getter!(username, String);
    getter!(password, String);

    pub fn is_anonymous(&self) -> bool {
        self.username().is_empty() && self.password().is_empty()
    }

    pub fn new(file_name: &str) -> Result<Self, std::io::Error> {
        match std::fs::read_to_string(file_name) {
            Ok(content) => Ok(content.as_str().into()),
            Err(err) => Err(err),
        }
    }
}

impl std::fmt::Debug for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.is_anonymous() {
            true => write!(f, "AppConfig(Anonymous)"),
            false => write!(
                f,
                "AppConfig(ClientID: {}, Username: {})",
                self.client_id(),
                self.username()
            ),
        }
    }
}

impl From<&str> for AppConfig {
    fn from(contents: &str) -> Self {
        toml::from_str(contents).unwrap_or_else(|err| {
            error!("Failed to create app config from file [{}]", err);
            std::process::exit(2)
        })
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
        let uname: String = conf.username.unwrap_or("".into());
        assert_eq!(uname, "test-user")
    }
}
