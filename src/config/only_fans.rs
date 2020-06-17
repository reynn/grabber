use serde_derive::{Deserialize, Serialize};

#[serde(default)]
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct OnlyFans {
    pub output_path: Option<String>,
    pub app_token: String,
    pub auth_id: String,
    pub auth_hash: String,
    pub session_id: String,
    pub users: Vec<String>,
}

impl std::fmt::Debug for OnlyFans {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Auth(id={})[users({})]", self.auth_id, self.users.len())
    }
}

impl From<&str> for OnlyFans {
    fn from(contents: &str) -> Self {
        toml::from_str(contents).unwrap_or_else(|err| {
            error!("Failed to create only_fans config from file [{}]", err);
            std::process::exit(2)
        })
    }
}