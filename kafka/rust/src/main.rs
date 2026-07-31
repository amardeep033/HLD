mod config;
mod consumer;
mod models;
mod producer;
mod routes;
mod state;

use axum::Router;
use std::net::SocketAddr;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let config = config::AppConfig::from_env();
    let state = state::build_app_state(&config).await?;
    let app = Router::new()
        .merge(routes::health::router())
        .merge(routes::orders::router())
        .with_state(state);

    let addr: SocketAddr = config.http_addr.parse()?;
    tracing::info!(%addr, "starting server");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            "kafka_rust=info,tower_http=info,rdkafka=warn",
        ))
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();
}
