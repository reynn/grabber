use grabber::config::AppConfig;

fn main() {
    let config: AppConfig = AppConfig::new(".grabber.toml").unwrap_or_else(|err| {
        eprintln!("Failed to create app configuration [{}]", err);
        std::process::exit(2);
    });

    if let Err(e) = grabber::start(config) {
        eprintln!("Failed to grab from Reddit due to error [{:#?}]", e);
        std::process::exit(1);
    }
}
