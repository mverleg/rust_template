
mod launch;
mod dependencies;

use launch::settings::Settings;

pub fn main() {
    // Read configuration from files and environment.
    let settings = Settings::new();
    //TODO @mark: log:
//    info!("{:?}", settings);
    println!("{:?}", settings);

}
