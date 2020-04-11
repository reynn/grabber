use rawr::structures::submission::Submission;
use reqwest::Url;
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct Downloadable {
    pub url: Url,
    pub title: String,
    pub user: String,
}

impl Downloadable {
    pub fn download(&self, out_path: &Path) -> Result<(), Box<dyn Error>> {
        let out_path = &out_path.join(&self.user);
        let output_file = &out_path.join(&self.url.path()[1..]);
        println!("[Download (url)] {}", self.url);
        println!("[Download (output_file): {:#?}", &output_file);
        let mut resp = reqwest::blocking::get(self.url.as_ref())?;
        let mut out_file = File::create(&output_file)?;
        match io::copy(&mut resp, &mut out_file) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}

impl From<&Submission<'_>> for Downloadable {
    fn from(submission: &Submission) -> Self {
        let sub_url = Url::parse(&submission.link_url().unwrap().as_str()).unwrap();
        Self {
            url: sub_url,
            title: submission.title().into(),
            user: "".into(),
        }
    }
}
