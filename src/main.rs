#![deny(warnings)]

mod options;
mod script;

use std::fs::File;
use std::path::PathBuf;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn init_logging() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

async fn load_script(path: impl Into<PathBuf>) -> anyhow::Result<script::Script> {
    let path = path.into();

    info!("Loading script: {:?}", path);
    let script = tokio::task::spawn_blocking(move || -> anyhow::Result<script::Script> {
        let file = File::open(path)?;
        let script = serde_json::from_reader(file)?;
        Ok(script)
    })
    .await??;

    Ok(script)
}

async fn run_script(
    connection_info: impl AsRef<str>,
    script: script::Script,
) -> anyhow::Result<()> {
    info!("Connecting redis: {}", connection_info.as_ref());

    let client = redis::Client::open(connection_info.as_ref())?;
    let mut con = client.get_async_connection().await?;

    info!("Running script...");

    let result: redis::Value = script.invoke_async(&mut con).await?;
    info!("Results:\n{:?}", result);

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging()?;

    let options: options::Options = argh::from_env();

    let script = load_script(&options.script).await?;
    run_script(options.connection_info(), script).await?;

    Ok(())
}
