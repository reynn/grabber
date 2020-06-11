#![warn(missing_debug_implementations, rust_2018_idioms)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;

mod collectors;
pub mod config;
mod download;
mod filters;

use crate::{collectors::only_fans::OnlyFansCollector, collectors::reddit::RedditCollector, download::DownloadManager};
use futures::join;

error_chain! {
    links {
        DownloadError(download::Error, download::ErrorKind);
        CollectorError(collectors::errors::Error, collectors::errors::ErrorKind);
    }
    foreign_links {
        Io(std::io::Error);
        TOMLSerializeError(toml::ser::Error);
        TOMLDeserializeError(toml::de::Error);
    }
}

pub async fn start<'a>(config: &'a config::AppConfig) -> Result<()> {
    let download_manager = DownloadManager::new(config.output_path.as_str(), 50);
    let download_async = download_manager.download_items();

    let reddit = RedditCollector::new(&config)?;
    let only_fans = OnlyFansCollector::new(&config)?;

    let red_collect = collectors::run_collector(reddit);
    let of_collect = collectors::run_collector(only_fans);

    let (red_complete, of_complete) = join!(red_collect, of_collect);
    if let Err(red_err) = red_complete {
        error!("Failed to run Reddit collector {}", red_err);
    }
    if let Err(of_err) = of_complete {
        error!("Failed to run OnlyFans collector {}", of_err);
    }

    info!("Collectors completed successfully");
    download_async.await?;
    info!("Downloads complete");

    Ok(())
}
