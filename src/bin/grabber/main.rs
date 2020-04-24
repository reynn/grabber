use clap::Clap;
use log::{debug, error, info};
use simplelog::*;
use std::process::exit;

use grabber::config::AppConfig;

#[derive(Debug, Clone, Default, Clap)]
#[clap(version = "0.1.0", author = "reynn")]
struct Opts {
    #[clap(short = "c", long = "config", default_value = ".grabber.toml")]
    config: String,
    #[clap(short = "v", long = "verbose")]
    verbose: bool,
}

fn main() {
    let opts: Opts = Opts::parse();
    let start = std::time::Instant::now();

    let log_level = match opts.verbose {
        true => {
            println!("turning on debug logging");
            LevelFilter::Debug
        }
        false => LevelFilter::Info,
    };

    CombinedLogger::init(vec![
        TermLogger::new(log_level, Config::default(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            std::fs::File::create("grabber.log").unwrap(),
        ),
    ])
    .unwrap();

    let config = AppConfig::new(opts.config.as_str()).unwrap_or_else(|err| {
        error!("Failed to create app configuration [{}]", err);
        exit(2);
    });

    debug!(
        "[grabber (config)] took {} ms",
        &start.elapsed().as_millis()
    );

    match grabber::start(config) {
        Ok(_) => info!(
            "grabber complete, took {} seconds",
            &start.elapsed().as_secs()
        ),
        Err(err) => {
            eprintln!("Failed to run grabber loop {}", err);
            std::process::exit(2);
        }
    }
}
