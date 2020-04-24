use reqwest::Url;
use std::{error::Error, fs::File, io, path::Path};

#[derive(Debug)]
pub struct Downloadable {
    pub url: Url,
    pub title: String,
    pub post_id: String,
    pub user: String,
}

impl Downloadable {
    pub fn new(url: Url, post_id: String, title: String, user: String) -> Self {
        Self {
            post_id,
            url,
            title,
            user
        }
    }
    pub fn download(&self, out_path: &Path) -> Result<String, Box<dyn Error>> {
        // path.join doesn't work if the join starts with a /
        // url.path() returns the leading / all the time, probably a better way to handle this
        let output_file = &out_path.join(&self.user).join(&self.url.path()[1..]);
        info!("[Download] {} -> {:?}", self.url, &output_file);
        let mut resp = reqwest::blocking::get(self.url.as_ref())?;
        let mut out_file = File::create(&output_file)?;
        match io::copy(&mut resp, &mut out_file) {
            Ok(_) => Ok(String::from("")),
            Err(e) => Err(Box::new(e)),
        }
    }
}
