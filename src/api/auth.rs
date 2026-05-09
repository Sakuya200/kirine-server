use axum::response::IntoResponse;

pub async fn login() -> impl IntoResponse {
    "login"
}

pub async fn logout() -> impl IntoResponse {
    "logout"
}