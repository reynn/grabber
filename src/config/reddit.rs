use serde_derive::{Deserialize, Serialize};

#[serde(default)]
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Reddit {
    pub output_path: Option<String>,
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
            write!(f, "Reddit(Anonymous)[users({})]", self.users.len())
        } else {
            write!(
                f,
                "Reddit(ClientID: {}, Username: {})[users({})]",
                self.client_id,
                self.username,
                self.users.len()
            )
        }
    }
}

impl From<&str> for Reddit {
    fn from(contents: &str) -> Self {
        toml::from_str(contents).unwrap_or_else(|err| {
            error!("Failed to create app config from file [{}]", err);
            std::process::exit(2)
        })
    }
}
