use anyhow::Result;
use tracing::*;

use so_far_so_good::common;
use so_far_so_good::config::Config;
use so_far_so_good::server;

#[tokio::main]
async fn main() -> Result<()> {
    info!("The story starts from here.");
    let config_path = common::init();

    let config = Config::new(config_path)?;

    server::start(config).await;

    info!("It's a happy ending.");
    Ok(())
}
