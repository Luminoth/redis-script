//#![deny(warnings)]

mod options;
mod script;

use std::fs::File;
use std::path::Path;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn init_logging() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

fn load_script(path: impl AsRef<Path>) -> anyhow::Result<script::Script> {
    let file = File::open(path)?;
    let script = serde_json::from_reader(file)?;

    Ok(script)
}

fn run_script(script: script::Script) -> anyhow::Result<()> {
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging()?;

    let options: options::Options = argh::from_env();

    let script = load_script(&options.script)?;
    run_script(script)?;

    Ok(())
}
