#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;

use clap::Clap;
use grabber::config::AppConfig;
use log::LevelFilter;
use std::process::exit;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        FernInitLogging(fern::InitError);
        FernSetupLogging(log::SetLoggerError);
        TOMLSerializeError(toml::ser::Error);
        TOMLDeserializeError(toml::de::Error);
        RawrError(rawr::errors::APIError);
    }
}

#[cfg(debug_assertions)]
static GRABBER_DEFAULT: &str = ".grabber.dev.toml";
#[cfg(not(debug_assertions))]
static GRABBER_DEFAULT: &str = ".grabber.toml";

#[derive(Debug, Clone, Default, Clap)]
#[clap(version = "0.1.0", author = "reynn")]
struct Opts {
    #[clap(short = "c", long = "config", default_value = GRABBER_DEFAULT)]
    config: String,
    #[clap(short = "v", long = "verbose")]
    verbose: bool,
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

    let log_level = if opts.verbose {
        println!("turning on debug logging");
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    setup_logging(log_level).unwrap();

    let config = AppConfig::new(opts.config.as_str()).unwrap_or_else(|err| {
        error!("Failed to create app configuration [{}]", err);
        exit(2);
    });

    debug!("took {} ms to load config", &start.elapsed().as_millis());

    match grabber::start(&config).await {
        Ok(_) => info!("grabber complete, took {} seconds", &start.elapsed().as_secs()),
        Err(err) => {
            eprintln!("Failed to run grabber loop {}", err);
            std::process::exit(2);
        }
    }
}
