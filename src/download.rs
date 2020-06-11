use reqwest::{Client, Url};
use std::{borrow::Borrow, fs::File, io::prelude::*, path::Path};
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

#[derive(Debug)]
pub struct DownloadManager<'a> {
    client: Client,
    out_path: &'a Path,
    buffer_size: usize,
    pub send_chan: Sender<DownloadItem>,
    pub recv_chan: Receiver<DownloadItem>,
}

impl<'a> Default for DownloadManager<'a> {
    fn default() -> Self {
        Self::new(".", 50)
    }
}

impl<'a> DownloadManager<'a> {
    pub fn new(path: &'a str, buff_size: usize) -> Self {
        let (s, r): (Sender<DownloadItem>, Receiver<DownloadItem>) = bounded(buff_size);
        Self {
            client: Client::new(),
            buffer_size: buff_size,
            out_path: Path::new(path),
            send_chan: s,
            recv_chan: r,
        }
    }

    pub async fn download_items(&self) -> Result<()> {
        loop {
            select! {
              recv(self.recv_chan) -> downloadable => {
                  downloadable?.get(self.out_path, &self.client).await?;
              },
              default(std::time::Duration::from_secs(2)) => {
                  info!("Timed out waiting for new downloadable");
                  break
              },
            }
        }
        Ok(())
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
        Self {
            collector_name,
            url,
            sub_path,
        }
    }

    async fn get(&self, out_path: &Path, client: &Client) -> Result<()> {
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
        let mut out_file = File::create(&output_file)?;
        let body_bytes = resp.text().await?.into_bytes();

        out_file.write_all(&body_bytes[..])?;
        info!("{} -> {:?}", self.url, &output_file);
        Ok(())
    }
}
