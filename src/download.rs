use rawr::structures::submission::Submission;
use reqwest::Url;
use std::{error::Error, fs::File, io, path::Path};

#[derive(Debug)]
pub struct Downloadable {
    pub url: Url,
    pub title: String,
    pub user: String,
}

impl Downloadable {
    pub fn download(&self, out_path: &Path) -> Result<String, Box<dyn Error>> {
        // path.join doesn't work if the join starts with a /
        // url.path() returns the leading / all the time, probably a better way to handle this
        let output_file = &out_path.join(&self.user).join(&self.url.path()[1..]);
        debug!("[Downloadable (url)] {}", self.url);
        debug!("[Downloadable (output_file): {:?}", &output_file);
        let mut resp = reqwest::blocking::get(self.url.as_ref())?;
        let mut out_file = File::create(&output_file)?;
        match io::copy(&mut resp, &mut out_file) {
            Ok(_) => Ok(String::from("")),
            Err(e) => Err(Box::new(e)),
        }
    }
}

impl From<&Submission<'_>> for Downloadable {
    fn from(submission: &Submission<'_>) -> Self {
        let sub_url = Url::parse(&submission.link_url().unwrap().as_str()).unwrap();
        Self {
            url: sub_url,
            title: submission.title().into(),
            user: "".into(),
        }
    }
}
