use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderMap, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use crate::api::entity::CommonResponse;
use crate::service::AppState;
use anyhow::Result;
use axum::{Json, Router};
use axum::routing::post;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use tracing::error;

#[derive(Clone, Debug)]
pub struct JwtParser {
    pub issuer: String,
    pub audience: String,
    pub public_key: DecodingKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    iss: String,
    aud: String,
    exp: usize,
    nbf: Option<usize>,
    iat: Option<usize>,
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("missing bearer token")]
    MissingToken,
    #[error("invalid bearer token")]
    InvalidToken,
    #[error("forbidden")]
    Forbidden,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            AuthError::MissingToken | AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),
        };
        CommonResponse::<()>::error(code.as_u16() as i32, msg).into_response()
    }
}


#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn login(State(app_state): State<AppState>, Json(request): Json<LoginRequest>) -> CommonResponse<String> {
    CommonResponse::success(Some("login".to_string()))
}

pub async fn logout(State(app_state): State<AppState>) -> CommonResponse<String> {
    CommonResponse::success(Some("logout".to_string()))
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AuthError> {
    let token = extract_bearer(req.headers()).ok_or(AuthError::MissingToken)?;

    let claims = state.jwt_validator
        .parse_token(token)
        .map_err(|_| AuthError::InvalidToken)?;

    // 注入 claims 给下游 handler
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

fn extract_bearer(headers: &HeaderMap) -> Option<&str> {
    headers.get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer ").map(|t| t.trim()))
}

impl JwtParser {

    pub fn parse_token(&self, token: &str) -> Result<Claims> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[self.issuer.as_str()]);
        validation.set_audience(&[self.audience.as_str()]);

        let data = decode::<Claims>(token, &self.public_key, &validation);
        if let Err(e) = data {
            error!("Failed to decode token: {e}");
            return Err(AuthError::InvalidToken.into());
        }

        Ok(data?.claims)
    }

    pub fn new(public_key_pem: String) -> Self {
        let public_key = DecodingKey::from_rsa_pem(public_key_pem.as_bytes())
            .expect("Auth config loaded failed: Failed to create DecodingKey from public key PEM");

        JwtParser {
            issuer: "kirine-server".to_string(),
            audience: "kirine-api".to_string(),
            public_key,
        }
    }
}