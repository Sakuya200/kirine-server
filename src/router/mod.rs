use axum::Router;
use axum::routing::post;
use crate::api::auth::{auth_middleware, login, logout};
use crate::config::EnvConfig;
use crate::service::AppState;
use crate::storage::LocalStorage;

pub fn init_router(env_config: EnvConfig, local_storage: LocalStorage) -> Router {
    let app_state = AppState::new(env_config, local_storage);

    Router::new()
        .nest("/auth", auth_api(app_state.clone()))
        .with_state(app_state)
}


pub fn auth_api(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/logout", post(logout))
        .route_layer(axum::middleware::from_fn_with_state(app_state, auth_middleware))
        .route("/login", post(login))
}