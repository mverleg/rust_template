mod dependencies;
mod launch;

use launch::settings::Settings;
use launch::setup;

pub fn main() {
    println!("built with {}", env!("VCS_SUMMARY"));
    // Read configuration from files and environment.
    let settings = Settings::new();
    //TODO @mark: log:
    //    info!("{:?}", settings);
    println!("{:?}", settings);
    setup(&settings);
}
