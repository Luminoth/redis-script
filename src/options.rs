use std::path::PathBuf;

use argh::FromArgs;

/// Redis script runner
#[derive(FromArgs, Debug)]
pub struct Options {
    /// redis host
    #[argh(option, short = 'h', default = "String::from(\"127.0.0.1\")")]
    host: String,

    /// redis port
    #[argh(option, short = 'p', default = "6379")]
    port: u16,

    /// script to run
    #[argh(positional)]
    pub script: PathBuf,
}

impl Options {
    pub fn connection_info(&self) -> String {
        format!("redis://{}:{}/", self.host, self.port)
    }
}
