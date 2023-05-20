use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tracing::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: Server,
    pub auth: Option<Auth>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub addr: SocketAddr,
    pub timeout: Duration,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Auth {
    pub addr: String,
}

pub type SharedConfig = Arc<Config>;

impl Config {
    pub fn new<T: AsRef<Path>>(config_path: T) -> Result<SharedConfig> {
        info!("Load config");
        Ok(Arc::new(serde_yaml::from_str(&fs::read_to_string(
            config_path,
        )?)?))
    }
}
