/// An error with the configuration file
#[allow(dead_code)]
pub enum AppConfigError {
    /// A valid configuration file exists but a value is missing
    MissingConfiguration(String),
    /// A valid configuration file exists but there are multiple missing values
    MissingConfigurations(Vec<String>),
    /// A configuration file is missing
    MissingConfigFile(String),
}
