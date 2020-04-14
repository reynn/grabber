use clap::Clap;
use grabber::config::AppConfig;
use log::{error, warn};
use simplelog::*;

#[derive(Debug, Clone, Default, Clap)]
#[clap(version = "0.1.0", author = "reynn")]
struct Opts {
    #[clap(short = "c", long = "config", default_value = ".grabber.toml")]
    config: String,
}

fn main() {
    if TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed).is_err() {
        warn!("Failed to configure TermLogger");
        SimpleLogger::init(LevelFilter::Info, Config::default())
            .expect("No logger should be already set");
    }

    let opts: Opts = Opts::parse();

    let config = AppConfig::new(opts.config.as_str()).unwrap_or_else(|err| {
        error!("Failed to create app configuration [{}]", err);
        std::process::exit(2);
    });

    match grabber::start(config) {
        Ok(_) => (),
        Err(err) => {
            error!("Failed to grab from Reddit due to error [{:#?}]", err);
            std::process::exit(1);
        }
    }
}
