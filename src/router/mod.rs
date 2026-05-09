use axum::Router;
use axum::routing::get;
use crate::{health, root};

pub fn init_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
}