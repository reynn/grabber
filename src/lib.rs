#![warn(missing_debug_implementations, rust_2018_idioms)]

#[macro_use]
extern crate log;

pub mod collectors;
pub mod config;
pub mod download;
pub mod filters;

use anyhow::Result;

use crate::{
    collectors::{Collectors, only_fans::Collector as ofc, fantia_jp::Collector as fc, reddit::Collector as rc},
    download::Manager,
};

pub async fn start(config: config::AppConfig) -> Result<()> {
    info!("Config: {:?}", config);
    let manager = Manager::new(config.general.output_path.as_str(), 50);
    let download_async = manager.download_items();

    let available_collectors = Collectors {
        collectors: vec![
            Box::new(rc::new(config.clone())?),
            Box::new(ofc::new(config.clone())?),
            Box::new(fc::new(config.clone())?),
        ],
    };

    available_collectors.run(&config, manager.send_chan.clone()).await?;

    info!("Collectors completed successfully");
    info!("Waiting for all downloads to complete...");
    download_async.await?;
    info!("Downloads complete");

    Ok(())
}
