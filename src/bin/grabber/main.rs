use clap::Clap;
use grabber::config::AppConfig;
use log::{error, warn};
use simplelog::*;
use std::process::exit;

#[derive(Debug, Clone, Default, Clap)]
#[clap(version = "0.1.0", author = "reynn")]
struct Opts {
    #[clap(short = "c", long = "config", default_value = ".grabber.toml")]
    config: String,
    #[clap(short = "v", long = "verbose")]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let log_level = match opts.verbose {
        true => LevelFilter::Debug,
        false => LevelFilter::Info,
    };

    if TermLogger::init(log_level, Config::default(), TerminalMode::Mixed).is_err() {
        warn!("Failed to configure TermLogger");
        SimpleLogger::init(log_level, Config::default()).expect("No logger should be already set");
    };

    let config = AppConfig::new(opts.config.as_str()).unwrap_or_else(|err| {
        error!("Failed to create app configuration [{}]", err);
        exit(2);
    });

    grabber::start(config)
}
