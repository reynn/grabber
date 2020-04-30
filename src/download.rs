use reqwest::{blocking::Client, Url};
use std::borrow::Borrow;
use std::{fs::File, io, path::Path};

#[derive(Debug)]
pub struct DownloadManager<'a> {
    client: Client,
    out_path: &'a Path,
}
impl<'a> Default for DownloadManager<'a> {
    fn default() -> Self {
        Self::new(".")
    }
}

impl<'a> DownloadManager<'a> {
    pub fn new(path: &'a str) -> Self {
        Self {
            client: Client::new(),
            out_path: Path::new(path),
        }
    }

    pub fn download_items(&self, items: &Vec<DownloadItem>) {
        for download_item in items.iter() {
            if let Err(e) = download_item.download(self.out_path, &self.client) {
                error!("MANAGER Failed to download {:#?}", e)
            }
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq)]
pub struct DownloadItem {
    pub url: Url,
    pub post_id: String,
    pub user: String,
}

impl PartialEq for DownloadItem {
    fn eq(&self, other: &Self) -> bool {
        self.url.eq(other.url.borrow())
    }

    fn ne(&self, other: &Self) -> bool {
        !self.url.eq(other.url.borrow())
    }
}

impl DownloadItem {
    pub fn new(url: Url, post_id: String, user: String) -> Self {
        Self { post_id, url, user }
    }

    fn download(&self, out_path: &Path, client: &Client) -> Result<(), std::io::Error> {
        // path.join doesn't work if the join starts with a /
        // url.path() returns the leading / all the time, probably a better way to handle this
        let output_file = &out_path
            .join(&self.user)
            .join(
                &self
                    .url
                    .domain()
                    .expect("Shouldn't get to this point without a URL"),
            )
            .join(&self.url.path()[1..]);
        if !&output_file.exists() {
            if let Err(e) = std::fs::create_dir_all(
                &output_file
                    .parent()
                    .expect("Has to have at least 2 parents, unsure how this happened"),
            ) {
                error!("Failed to create parent folder [{}]", e)
            }
        }
        info!("[Download] {} -> {:?}", self.url, &output_file);
        if let Ok(ref mut resp) = client.get(self.url.as_str()).send() {
            if let Ok(ref mut out_file) = File::create(&output_file) {
                if let Err(copy_err) = io::copy(resp, out_file) {
                    error!(
                        "Failed to write file [{:?}] error {:?}",
                        &output_file, copy_err
                    );
                }
            }
        }
        Ok(())
    }
}
