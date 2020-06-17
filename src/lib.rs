#![warn(missing_debug_implementations, rust_2018_idioms)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;

pub mod collectors;
pub mod config;
pub mod download;
pub mod filters;

use crate::{
    collectors::{only_fans::OnlyFansCollector, fantia_jp::FantiaCollector, reddit::RedditCollector},
    download::Manager,
};
use futures::join;

error_chain! {
    links {
        DownloadError(download::errors::Error, download::errors::ErrorKind);
        CollectorError(collectors::errors::Error, collectors::errors::ErrorKind);
    }
    foreign_links {
        Io(std::io::Error);
        TOMLSerializeError(toml::ser::Error);
        TOMLDeserializeError(toml::de::Error);
    }
}

pub async fn start<'a>(config: &'a config::AppConfig) -> Result<()> {
    info!("Config: {:?}", config);
    let manager = Manager::new(config.general.output_path.as_str(), 50);
    let download_async = manager.download_items();

    let reddit = RedditCollector::new(&config)?;
    let red_collect = collectors::run_collector(reddit, manager.send_chan.clone());

    let fantia = FantiaCollector::new(&config)?;
    let fantia_collect = collectors::run_collector(fantia, manager.send_chan.clone());

    let only_fans = OnlyFansCollector::new(&config)?;
    let of_collect = collectors::run_collector(only_fans, manager.send_chan.clone());

    let (fantia_complete, of_complete, red_complete) = join!(fantia_collect, of_collect, red_collect);
    if let Err(reddit_err) = red_complete {
        error!("Failed to run Reddit collector {}", reddit_err);
    }
    if let Err(fantia_err) = fantia_complete {
        error!("Failed to run Fantia collector {}", fantia_err);
    }
    if let Err(of_err) = of_complete {
        error!("Failed to run OnlyFans collector {}", of_err);
    }

    info!("Collectors completed successfully");
    info!("Waiting for all downloads to complete...");
    download_async.await?;
    info!("Downloads complete");

    Ok(())
}
