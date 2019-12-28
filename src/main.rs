extern crate blas;
extern crate openblas_src;

mod dependencies;
mod launch;

use crate::launch::settings::Settings;
use crate::launch::setup;

use ::pretty_env_logger;

pub fn main() {
    pretty_env_logger::init();
    println!("built with {}", env!("VCS_SUMMARY"));
    // Read configuration from files and environment.
    let settings = Settings::new();
    //TODO @mark: log:
    //    info!("{:?}", settings);
    println!("{:?}", settings);
    setup(&settings);
}
