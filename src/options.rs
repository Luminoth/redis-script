use std::path::PathBuf;
use std::sync::Arc;

use argh::FromArgs;
use parking_lot::RwLock;

/// Website API
#[derive(FromArgs, Debug)]
pub struct Options {
    /// redis host
    #[argh(option, short = 'h', default = "String::from(\"127.0.0.1\")")]
    pub host: String,

    /// redis port
    #[argh(option, short = 'p', default = "6379")]
    pub port: u16,

    /// script to run
    #[argh(option)]
    script: PathBuf,
}

pub type SharedOptions = Arc<RwLock<Options>>;
