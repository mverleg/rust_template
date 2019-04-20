use std::env;
use std::path::PathBuf;

mod launch;
mod dependencies;

use launch::settings::Settings;

const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");

pub fn main() {
    // Read configuration from files and environment.
    let settings = launch::settings::Settings::new();
    //TODO @mark: log:
//    info!("{:?}", settings);

    let (server_config, _remaining_args) = {
        let dirs = ProjectDirs::from("", CRATE_NAME, CRATE_NAME).unwrap();
        let mut cwd_pth = env::current_dir().unwrap();
        cwd_pth.push("config.yaml");
        let mut paths = vec![
            dirs.config_dir().to_owned(),
            cwd_pth,
        ];
        //TODO: Would be nice if this path included CRATE_NAME (to_upper), but only literals can be used
        if let Some(pth) = option_env!("CONFIG_PATH") {
            let pth = PathBuf::from(pth);
            if pth.exists() {
                paths.push(pth);
            }
        }
        Config::including_optional_config_files(&paths).unwrap_or_exit()
        // TODO: does not include the REPO_ROOT/config dir (but then, that shouldn't be checked in)
    };
}
