pub mod item;

use self::item::Item;

use reqwest::Client;
use std::path::Path;
use crossbeam_channel::{bounded, select, Sender, Receiver};
use anyhow::Result;

#[derive(Debug)]
pub struct Manager<'a> {
    client: Client,
    out_path: &'a Path,
    buffer_size: usize,
    pub send_chan: Sender<Item>,
    pub recv_chan: Receiver<Item>,
}

impl<'a> Default for Manager<'a> {
    fn default() -> Self {
        Self::new(".", 50)
    }
}

impl<'a> Manager<'a> {
    pub fn new(path: &'a str, buff_size: usize) -> Self {
        let (s, r): (Sender<Item>, Receiver<Item>) = bounded(buff_size);
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
