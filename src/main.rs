mod dependencies;

use configure_me::include_config;
use directories::ProjectDirs;
use std::env;

const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");
include_config!();

pub fn main() {
    // Read configuration from files and environment.
    let (server_config, _remaining_args) = {
        let dirs = ProjectDirs::from("", CRATE_NAME, CRATE_NAME);
        Config::including_optional_config_files(&[
            dirs.config_dir(),
            env::current_dir().push("config.yaml"),
            env!(format!("{}_CONFIG_DIR", CRATE_NAME))
        ]).unwrap_or_exit()
        // TODO: does not include the REPO_ROOT/config dir (but then, that shouldn't be checked in)
    };

}
