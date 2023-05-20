use anyhow::Result;
use tracing::*;

use so_far_so_good::common;
use so_far_so_good::config::Config;
use so_far_so_good::healthcheck::{self, Healthcheck};
use so_far_so_good::server;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().json().init();
    info!("The story starts from here.");

    let config_path = common::init();

    let config = Config::new(config_path)?;

    let healthcheck = Healthcheck::new();

    let fut_monitoring = tokio::spawn(healthcheck::monitoring(healthcheck.clone()));

    let fut_server = tokio::spawn(server::start(config, healthcheck));

    if let Err(e) = tokio::try_join!(fut_monitoring, fut_server) {
        error!("the magic is broken.: {:?}", e);
    }

    info!("It's a happy ending.");
    Ok(())
}
