//#![deny(warnings)]

mod options;

use std::sync::Arc;

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::options::SharedOptions;

static OPTIONS: Lazy<SharedOptions> = Lazy::new(|| Arc::new(RwLock::new(argh::from_env())));

fn init_logging() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging()?;

    info!("Hello, world!");

    Ok(())
}
