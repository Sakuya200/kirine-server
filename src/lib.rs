use std::net::SocketAddr;
use crate::router::init_router;

mod router;
mod config;
mod storage;
mod service;
mod api;

pub async fn start_server() {
    let router = init_router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind 127.0.0.1:3000");

    println!(
        "listening on {}",
        listener.local_addr().expect("invalid local addr")
    );

    axum::serve(listener, router)
        .await
        .expect("server failed unexpectedly");
}