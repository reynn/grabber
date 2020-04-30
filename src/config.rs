use serde_derive::{Deserialize, Serialize};

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
#[derive(Default, Deserialize, Serialize)]
pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
    pub output_path: String,
    pub users: Vec<String>,
    pub friend_manage: bool,
}

impl AppConfig {
    pub fn is_anonymous(&self) -> bool {
        self.username.is_empty() && self.password.is_empty()
    }

    pub fn new(file_name: &str) -> Result<Self> {
        let content = std::fs::read_to_string(file_name)?;
        Ok(content.as_str().into())
    }
}

impl std::fmt::Debug for AppConfig {
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
        let uname = conf.username;
        assert_eq!(uname, "test-user")
    }
}
