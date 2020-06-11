use serde_derive::{Deserialize, Serialize};
use crate::config::errors::*;

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Fantia {
  pub fan_clubs: Vec<String>,
}

impl std::fmt::Debug for Fantia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fantia[clubs({})]", self.fan_clubs.len())
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
