/// Code related to starting the application, such as (one day):
/// - Reading configuration
/// - Command line interface
/// - Database initialization

const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");

pub mod settings;

use human_panic::setup_panic;

pub fn setup(conf: &settings::Settings) {
    // Set up better panic messages.
    if conf.friendly_panics {
        setup_panic!();
    }
}
