use clap::Clap;
use grabber::config::AppConfig;

#[derive(Debug, Clone, Default, Clap)]
#[clap(version = "0.1.0", author = "Reynn")]
struct Opts {
    #[clap(short = "c", long = "config", default_value = ".grabber.toml")]
    config: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let config: AppConfig = AppConfig::new(opts.config.as_str()).unwrap_or_else(|err| {
        eprintln!("Failed to create app configuration [{}]", err);
        std::process::exit(2);
    });

    if let Err(e) = grabber::start(config) {
        eprintln!("Failed to grab from Reddit due to error [{:#?}]", e);
        std::process::exit(1);
    }
}
