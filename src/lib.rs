use crate::config::{init_config, init_log};
use crate::router::init_router;
use std::net::SocketAddr;
use tracing::info;

mod api;
mod config;
mod pipeline;
mod router;
mod service;
mod storage;
mod utils;

pub async fn start_server() {
    let env_config = init_config();
    let addr = SocketAddr::from(([127, 0, 0, 1], env_config.server.port));
    init_log(&env_config.log_dir());

    let router = init_router(env_config);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind 127.0.0.1:3000");

    info!(
        "listening on {}",
        listener.local_addr().expect("invalid local addr")
    );

    axum::serve(listener, router)
        .await
        .expect("server failed unexpectedly");
}
