use axum::Router;
use axum::routing::post;
use crate::api::auth::{login, logout};
use crate::config::EnvConfig;
use crate::service::AppState;

pub fn init_router(env_config: EnvConfig) -> Router {
    let app_state = AppState::new(env_config);

    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .with_state(app_state)
}