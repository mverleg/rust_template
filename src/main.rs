use std::env;
use std::path::PathBuf;

use configure_me::include_config;
use directories::ProjectDirs;

mod dependencies;

const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");
include_config!();

pub fn main() {
    // Read configuration from files and environment.
    let (server_config, _remaining_args) = {
        let dirs = ProjectDirs::from("", CRATE_NAME, CRATE_NAME).unwrap();
        let mut cwd_pth = env::current_dir().unwrap();
        cwd_pth.push("config.yaml");
        let mut paths = vec![
            dirs.config_dir(),
            &cwd_pth,
        ];
        println!("{:?}", format!("{}_CONFIG_DIR", CRATE_NAME));
//        if let Some(pth) = option_env!(format!("{}_CONFIG_DIR", CRATE_NAME)) {
//            if exists(pth) {
//                paths.push(pth);
//            }
//        }
        Config::including_optional_config_files(&paths).unwrap_or_exit()
        // TODO: does not include the REPO_ROOT/config dir (but then, that shouldn't be checked in)
    };
}
