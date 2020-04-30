#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;

use clap::Clap;
use simplelog::*;
use std::process::exit;

use grabber::config::AppConfig;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        TOMLSerializeError(toml::ser::Error);
        TOMLDeserializeError(toml::de::Error);
        RawrError(rawr::errors::APIError);
    }
}

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

    let log_level = if opts.verbose {
        println!("turning on debug logging");
        LevelFilter::Debug
    } else {
        LevelFilter::Info
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

    let mut config = AppConfig::new(opts.config.as_str()).unwrap_or_else(|err| {
        error!("Failed to create app configuration [{}]", err);
        exit(2);
    });

    debug!(
        "[grabber (config)] took {} ms",
        &start.elapsed().as_millis()
    );

    match grabber::start(&mut config) {
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
