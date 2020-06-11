use reqwest::{Client, Url};
use std::{borrow::Borrow, fs::File, io::prelude::*, path::Path};
use lazy_static::lazy_static;
use crossbeam_channel::{bounded, select, Sender, Receiver};
use rawr::structures::submission::Submission;

error_chain! {
    foreign_links {
        CrossbeamSendError(crossbeam_channel::SendError<DownloadItem>);
        CrossbeamReceiveError(crossbeam_channel::RecvError);
        IO(std::io::Error);
        RawrError(rawr::errors::APIError);
        ReqwestError(reqwest::Error);
    }
}

lazy_static!{
    static ref DOWNLOADER: (Sender<DownloadItem>, Receiver<DownloadItem>) = bounded(15);
}

pub fn send_download_item(di: DownloadItem) -> Result<()> {
  Ok(DOWNLOADER.0.send(di)?)
}

#[derive(Debug)]
pub struct DownloadManager<'a> {
    client: Client,
    out_path: &'a Path,
    buffer_size: usize,
    items: Vec<DownloadItem>,
}

impl<'a> Default for DownloadManager<'a> {
    fn default() -> Self {
        Self::new(".", 50)
    }
}

impl<'a> DownloadManager<'a> {
    pub fn new(path: &'a str, buff_size: usize) -> Self {
        Self {
            client: Client::new(),
            buffer_size: buff_size,
            out_path: Path::new(path),
            items: Vec::new(),
        }
    }

    pub async fn download_items(&self) -> Result<()> {
      loop {
        select!{
          recv(DOWNLOADER.1) -> downloadable => {
            downloadable?.get(self.out_path, &self.client).await?;
          }
        }
      }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq)]
pub struct DownloadItem {
    pub collector_name: String,
    pub url: Url,
    pub sub_path: Option<String>,
}

impl PartialEq for DownloadItem {
    fn eq(&self, other: &Self) -> bool {
        self.url.eq(other.url.borrow())
    }

    fn ne(&self, other: &Self) -> bool {
        !self.url.eq(other.url.borrow())
    }
}


impl From<Submission<'_>> for DownloadItem {
  fn from(s: Submission<'_>) -> Self {
    let url = reqwest::Url::parse(s.link_url().unwrap().as_str()).unwrap();
    Self {
      collector_name: "reddit".into(),
      url,
      sub_path: None,
    }
  }
}

impl DownloadItem {
    pub fn new(url: Url, collector_name: String, sub_path: Option<String>) -> Self {
        Self { collector_name, url, sub_path }
    }

    async fn get(&self, out_path: &Path, client: &Client) -> Result<()> {
        // path.join doesn't work if the join starts with a /
        // url.path() returns the leading / all the time, probably a better way to handle this
        let output_file = match &self.sub_path {
          Some(s_p) => {
            out_path
            .join(s_p)
            .join(&self.url.domain().expect("Shouldn't get to this point without a URL"))
            .join(&self.url.path()[1..])
          },
          None => {
            out_path
            .join(&self.url.domain().expect("Shouldn't get to this point without a URL"))
            .join(&self.url.path()[1..])
          },
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
        let mut out_file = File::create(&output_file)?;
        let body_bytes = resp.text().await?.into_bytes();

        out_file.write_all(&body_bytes[..])?;
        info!("{} -> {:?}", self.url, &output_file);
        Ok(())
    }
}
