use std::convert::Into;
use std::env;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

use super::CRATE_NAME;

#[derive(Debug, Deserialize)]
pub struct Settings {
    run_mode: String,
    log_level: u8,
    //TODO @mark: replace by log level ^
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut conf = Config::new();

        // TODO @mark: Files are read in this order:
        // * user profile (e.g. inside ~/.config/)
        // * base settings (config.yaml)
        // * [mode].yaml (from RUN_MODE, i.e. development.yaml)
        // * local.yaml (do not check this into VCS)
        // * environment

        // Start with the "default" configuration file.
        conf.merge(File::with_name("src/launch/config.defaults.yaml"))?;

        // Add the base user config file.
        conf.merge(File::with_name("config/config.yaml").required(false))?;

        // Add in the current environment file ('development' by default)
        let mode = env::var("RUN_MODE").unwrap_or("development".into());
        conf.merge(File::with_name(&format!("config/{}.yaml", mode)).required(false))?;

        // Add in a local configuration file
        conf.merge(File::with_name("config/local.yaml").required(false))?;

        // Add in settings from the environment (with a prefix)
        conf.merge(Environment::with_prefix(CRATE_NAME))?;

        // You may also programmatically change settings
        conf.set("run_mode", mode)?;

        // You can deserialize (and thus freeze) the entire configuration as
        conf.try_into()
    }
}
