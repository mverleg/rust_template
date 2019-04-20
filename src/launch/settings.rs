use std::convert::Into;
use std::env;

use config::{Config, Environment, File};
use serde::Deserialize;
use dotenv::{dotenv, Error};

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

        //TODO @mark: create a method that constructs the paths and returns them
        //TODO @mark: low which do and do not exist

        // Start with the "default" configuration file.
        conf.merge(File::with_name("src/launch/config.defaults.yaml"))
            .unwrap_or_else(|err| panic!("Failed to read default configuration: {:?}", err));

        // Read from user configuration dir.
        let conf_path: String = ProjectDirs::from("", CRATE_NAME, CRATE_NAME).unwrap()
            .config_dir().to_string_lossy().into_owned();
        conf.merge(File::with_name(&conf_path).required(false))
            .unwrap_or_else(|err| panic!("Failed to read user configuration as {}: {:?}", conf_path, err));

        // Add the base user config file.
        conf.merge(File::with_name("config/config.yaml").required(false))
            .unwrap_or_else(|err| panic!("Failed to read config.yaml (base file): {:?}", err));

        // Add in the current environment file ('development' by default)
        let mode = env::var("RUN_MODE").unwrap_or("development".into());
        let mode_file_name = format!("config/{}.yaml", mode);
        conf.merge(File::with_name(&mode_file_name).required(false))
            .unwrap_or_else(|err| panic!("Failed to read {} (based on RUN_MODE): {:?}", mode_file_name, err));

        // Add in a local configuration file
        conf.merge(File::with_name("config/local.yaml").required(false))
            .unwrap_or_else(|err| panic!("Failed to read local.yaml (override file): {:?}", err));

        // Read .env file into environment
        match dotenv() {
            Ok(_) => (),
            Err(err) => match err {
                Error::Io(_) => () /* perhaps the .env file did not exist, which is okay */,
                Error::LineParse(msg) => panic!("Failed to parse a line in .env file: {:?}", msg),
                Error::EnvVar(msg) => panic!("Failed to load var from .env file: {:?}", msg),
            }
        }

        // Add in settings from the environment (with a prefix)
        conf.merge(Environment::with_prefix(CRATE_NAME))
            .unwrap_or_else(|err| panic!("Failed to read environment settings: {:?}", err));

        // You may also programmatically change settings
        conf.set("run_mode", mode).unwrap();

        // You can deserialize (and thus freeze) the entire configuration as
        conf.try_into()
            .unwrap_or_else(|err| panic!("Failed to read configuration: {:?}", err))
    }
}
