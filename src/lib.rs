#![warn(missing_debug_implementations, rust_2018_idioms)]

#[macro_use]
extern crate log;

pub mod collectors;
pub mod config;
pub mod download;
pub mod filters;

use anyhow::Result;

use crate::{
    collectors::{Collectors, only_fans::OnlyFansCollector, fantia_jp::FantiaCollector, reddit::RedditCollector},
    download::Manager,
};

pub async fn start(config: config::AppConfig) -> Result<()> {
    info!("Config: {:?}", config);
    let manager = Manager::new(config.general.output_path.as_str(), 50);
    let download_async = manager.download_items();

    let available_collectors = Collectors {
        collectors: vec![
            Box::new(OnlyFansCollector::new(config.clone())?),
            Box::new(FantiaCollector::new(config.clone())?),
            Box::new(RedditCollector::new(config.clone())?),
        ],
    };

    available_collectors.run(&config, manager.send_chan.clone()).await?;

    info!("Collectors completed successfully");
    info!("Waiting for all downloads to complete...");
    download_async.await?;
    info!("Downloads complete");

    Ok(())
}
