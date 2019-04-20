mod dependencies;

use configure_me::include_config;

const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");
include_config!();

pub fn main() {
    // Read configuration from files and environment.
    let (server_config, _remaining_args) = Config::including_optional_config_files(&[
        "config/config.yaml",
    ]).unwrap_or_exit();

}
