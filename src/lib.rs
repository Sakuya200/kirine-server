use crate::config::{init_config, init_log};
use crate::router::init_router;
use std::net::SocketAddr;
use sha2::{Digest, Sha256};
use tracing::info;
use crate::storage::{init_storage, AppMetaStorage, UserCredential};

mod api;
mod config;
mod pipeline;
mod router;
mod service;
mod storage;
mod utils;
mod data_models;

pub async fn start_server() {
    let env_config = init_config();
    let addr = SocketAddr::from(([127, 0, 0, 1], env_config.server.port));
    // 初始化日志
    init_log(&env_config.log_dir());
    // 初始化数据库连接
    let storage = init_storage(&env_config).await;
    // 更新用户账号信息
    let user_credential = UserCredential {
        username: env_config.credential.username.clone(),
        password_hash: encode_password(&env_config.credential.password),
    };
    storage.update_app_user_credential(&user_credential).await
        .expect("Failed to update app user credential");

    let router = init_router(env_config, storage);
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


fn encode_password(password: &str) -> String {
    // Keep hash format stable for value stored in app_meta.admin_password.
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let digest = hasher.finalize();
    format!("{:x}", digest)
}