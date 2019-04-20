use std::convert::Into;
use std::env;

use config::{Config, Environment, File};
use serde::Deserialize;

use super::CRATE_NAME;
use directories::ProjectDirs;

//TODO @mark: .env file
//TODO @mark: CLI clap

#[derive(Debug, Deserialize)]
pub struct Settings {
    run_mode: String,
    log_level: u8,
    //TODO @mark: replace by log level ^
}

impl Settings {
    pub fn new() -> Self {
        let mut conf = Config::new();

        // Start with the "default" configuration file.
        conf.merge(File::with_name("src/launch/config.defaults.yaml"))
            .unwrap_or_else(|err| panic!("Failed to parse default configuration: {:?}", err));

        // Read from user configuration dir.
        let conf_path: String = ProjectDirs::from("", CRATE_NAME, CRATE_NAME).unwrap()
            .config_dir().to_string_lossy().into_owned();
        conf.merge(File::with_name(&conf_path).required(false))
            .unwrap_or_else(|err| panic!("Failed to parse default configuration: {:?}", err));

        // Add the base user config file.
        conf.merge(File::with_name("config/config.yaml").required(false))
            .unwrap_or_else(|err| panic!("Failed to parse config.yaml (base file): {:?}", err));

        // Add in the current environment file ('development' by default)
        let mode = env::var("RUN_MODE").unwrap_or("development".into());
        let mode_file_name = format!("config/{}.yaml", mode);
        conf.merge(File::with_name(&mode_file_name).required(false))
            .unwrap_or_else(|err| panic!("Failed to parse {} (based on RUN_MODE): {:?}", mode_file_name, err));

        // Add in a local configuration file
        conf.merge(File::with_name("config/local.yaml").required(false))
            .unwrap_or_else(|err| panic!("Failed to parse local.yaml (override file): {:?}", err));

        // Add in settings from the environment (with a prefix)
        conf.merge(Environment::with_prefix(CRATE_NAME))
            .unwrap_or_else(|err| panic!("Failed to parse environment settings: {:?}", err));

        // You may also programmatically change settings
        conf.set("run_mode", mode).unwrap();

        // You can deserialize (and thus freeze) the entire configuration as
        conf.try_into()
            .unwrap_or_else(|err| panic!("Failed to read configuration: {:?}", err))
    }
}
