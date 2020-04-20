use rawr::structures::submission::Submission;
use reqwest::Url;
use std::{error::Error, fs::File, io, path::Path, sync::mpsc};

#[derive(Debug)]
pub struct Manager {
    input_channel: mpsc::Sender<Downloadable>,
    out_channel: mpsc::Receiver<Downloadable>,
}

impl Manager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        Manager {
            input_channel: tx,
            out_channel: rx,
        }
    }

    pub fn handle_downloads(&self, output_file: &Path) {
        for downloadable in &self.out_channel {
            info!("[Download Manager] Downloading {:?}", downloadable);
            if let Err(e) = downloadable.download(output_file) {
                error!("[Download Manager] Failed to download: {:?}", e);
            } else {
                info!("[Downloadable] {} -> {:?}", downloadable.url, &output_file);
            }
        }
    }

    pub fn add_to_queue(&self, d: Downloadable) -> Result<(), mpsc::SendError<Downloadable>> {
        self.input_channel.send(d)
    }
}

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
    fn from(submission: &Submission) -> Self {
        let sub_url = Url::parse(&submission.link_url().unwrap().as_str()).unwrap();
        Self {
            url: sub_url,
            title: submission.title().into(),
            user: "".into(),
        }
    }
}
