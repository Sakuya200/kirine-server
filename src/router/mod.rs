use axum::Router;
use axum::routing::post;
use crate::api::auth::{login, logout};

pub fn init_router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
}