use ::std::path::PathBuf;

use ::derive_getters::Getters;
use ::env_logger;
use ::structopt::StructOpt;
use ::tokio;

use ::{{crate_name}}lib::run;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[derive(Debug, StructOpt, Getters)]
#[structopt(
    name = "todo",
    about = "todo"
)]
pub struct Args {
    #[structopt(long = "files", short = "f", parse(from_os_str))]
    files: Vec<PathBuf>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let _args = Args::from_args();
    run().await
}
