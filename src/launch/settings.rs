use config::{Config, ConfigError, Environment, File};
use std::convert::Into;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    run_mode: String,
    log_level: u8,
    //TODO @mark: replace by log level ^
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut conf = Config::new();

        // Start with the "default" configuration file.
        conf.merge(File::with_name("src/launch/config.defaults.yaml"))?;

        // Add the base user config file.
        conf.merge(File::with_name("config/config.yaml").required(false))?;

        // Add in the current environment file ('development' by default)
        let mode = env::var("RUN_MODE").unwrap_or("development".into());
        conf.merge(File::with_name(&format!("config/{}.yaml", mode)).required(false))?;

        // Add in a local configuration file
        conf.merge(File::with_name("config/local.yaml").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        conf.merge(Environment::with_prefix("app"))?;

        // You may also programmatically change settings
        conf.set("database.url", "postgres://")?;

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", conf.get_bool("debug"));
        println!("database: {:?}", conf.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        conf.try_into()
    }
}
