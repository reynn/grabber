use std::{borrow::Borrow, io::prelude::*, path::Path};

use super::errors::*;

#[derive(Debug, Ord, PartialOrd, Eq)]
pub struct Item {
    pub collector_name: String,
    pub url: reqwest::Url,
    pub sub_path: Option<String>,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.url.eq(other.url.borrow())
    }

    fn ne(&self, other: &Self) -> bool {
        !self.url.eq(other.url.borrow())
    }
}

impl Item {
    pub fn new(url: reqwest::Url, collector_name: String, sub_path: Option<String>) -> Self {
        Self {
            collector_name,
            url,
            sub_path,
        }
    }

    pub async fn get(&self, out_path: &Path, client: &reqwest::Client) -> Result<()> {
        // path.join doesn't work if the join starts with a /
        // url.path() returns the leading / all the time, probably a better way to handle this
        let output_file = match &self.sub_path {
            Some(s_p) => out_path
                .join(s_p)
                .join(&self.url.domain().expect("Shouldn't get to this point without a URL"))
                .join(&self.url.path()[1..]),
            None => out_path
                .join(&self.url.domain().expect("Shouldn't get to this point without a URL"))
                .join(&self.url.path()[1..]),
        };
        if !&output_file.exists() {
            if let Err(e) = std::fs::create_dir_all(
                &output_file
                    .parent()
                    .expect("Has to have at least 2 parents, unsure how this happened"),
            ) {
                error!("Failed to create parent folder [{}]", e)
            }
        }
        let resp = client.get(self.url.as_str()).send().await?;
        let mut out_file = std::fs::File::create(&output_file)?;
        let body_bytes = resp.text().await?.into_bytes();

        out_file.write_all(&body_bytes[..])?;
        info!("{} -> {:?}", self.url, &output_file);
        Ok(())
    }
}
