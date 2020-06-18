use serde_derive::{Deserialize, Serialize};

#[serde(default)]
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Fantia {
    pub output_path: Option<String>,
    pub session_id: String,
    pub fan_clubs: Vec<String>,
    pub enabled: bool,
}

impl std::fmt::Debug for Fantia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.enabled {
            write!(f, "Fantia([clubs={}])", self.fan_clubs.len())
        } else {
            write!(f, "Fantia(disabled)")
        }
    }
}

impl From<&str> for Fantia {
    fn from(contents: &str) -> Self {
        toml::from_str(contents).unwrap_or_else(|err| {
            error!("Failed to create Fantia config from file [{}]", err);
            std::process::exit(2)
        })
    }
}
