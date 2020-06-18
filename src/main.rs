#[macro_use]
extern crate log;

use clap::Clap;
use log::LevelFilter;
use std::{process::exit, io::prelude::*};
use anyhow::Result;

use grabber::config::AppConfig;

static GRABBER_VERSION: &str = "0.1.0";

#[cfg(debug_assertions)]
static GRABBER_DEFAULT: &str = ".grabber.dev.toml";
#[cfg(not(debug_assertions))]
static GRABBER_DEFAULT: &str = ".grabber.toml";

#[derive(Debug, Clone, Clap)]
#[clap(version = GRABBER_VERSION, author = "reynn <nic@reynn.dev>")]
struct Opts {
    #[clap(short, long, default_value = GRABBER_DEFAULT)]
    config: String,
    #[clap(short, long)]
    verbose: bool,
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Clap, Debug, Clone)]
enum SubCommand {
    #[clap(version = GRABBER_VERSION, author = "reynn <nic@reynn.dev>")]
    Config(Config),
}

/// A subcommand for controlling testing
#[derive(Clap, Debug, Clone, Default)]
struct Config {
    /// Print debug info
    #[clap(short, default_value = GRABBER_DEFAULT)]
    path: String,
}

fn setup_logging(level: LevelFilter) -> Result<()> {
    Ok(fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .chain(fern::log_file("grabber.log")?)
        .apply()?)
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();
    let start = std::time::Instant::now();

    println!("opts :: {:#?}", opts);

    let log_level = match opts.verbose {
        true => LevelFilter::Debug,
        false => LevelFilter::Info,
    };

    setup_logging(log_level).unwrap();

    match opts.subcmd {
        Some(SubCommand::Config(c)) => {
            let app_config = AppConfig::default();
            let app_config = toml::to_string_pretty(&app_config).unwrap();
            let mut out_file = std::fs::File::create(c.path).unwrap();
            if let Err(err) = out_file.write(app_config.as_bytes()) {
                eprintln!("Failed to create config: {}", err);
                exit(2)
            }
        }
        None => {
            let config = AppConfig::new(opts.config.as_str()).unwrap_or_else(|err| {
                error!("Failed to create app configuration [{}]", err);
                exit(2);
            });

            match grabber::start(config).await {
                Ok(_) => info!("grabber complete, took {} seconds", &start.elapsed().as_secs()),
                Err(err) => {
                    eprintln!("Failed to run grabber loop {}", err);
                    std::process::exit(2);
                }
            }
        }
    }
}
