use axum::{http::StatusCode, response::IntoResponse, Json};
use clap::{arg, value_parser, Command};
use serde_json::json;
use std::path::PathBuf;

pub fn init() -> PathBuf {
    tracing_subscriber::fmt().json().init();

    let c = Command::new(clap::crate_name!())
        .author("ukimochi <mochi.in.the.sky.net@gmail.com>")
        .about("A websocket server that decodes and resamples the input sound source")
        .version(env!("VERGEN_GIT_SHA"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a config file"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    c.get_one::<PathBuf>("config").unwrap().to_path_buf()
}

pub async fn version() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json! {
            {
                "package_version": env!("CARGO_PKG_VERSION").to_string(),
                "package_hash": env!("VERGEN_GIT_SHA").to_string(),
                "package_features": env!("VERGEN_CARGO_FEATURES").to_string(),
            }
        }),
    )
}
