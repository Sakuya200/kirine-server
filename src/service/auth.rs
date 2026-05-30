use async_trait::async_trait;
use crate::api::auth::LoginRequest;
use anyhow::{bail, Context, Result};
use crate::service::AppState;
use crate::storage::{AppMetaStorage, UserCredential};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[async_trait]
pub trait AuthService {
    async fn login(&self, request: LoginRequest) -> Result<String>;
    async fn logout(&self) -> Result<()>;
}



#[async_trait]
impl AuthService for AppState {
    async fn login(&self, request: LoginRequest) -> Result<String> {
        let storage = self.storage.clone();
        let user_credential = storage.get_app_user_credential().await?;
        let input_password_hash = encode_password(request.password);
        if request.username != user_credential.username
            || !input_password_hash.eq(&user_credential.password_hash) {
            bail!("用户名或密码错误");
        }
        generate_token(self, &user_credential)
    }

    async fn logout(&self) -> Result<()> {
        bail!("目前暂不支持服务端登出功能，请直接删除客户端保存的 token 来实现登出");
    }
}

fn encode_password(password: String) -> String {
    // Keep hash format stable for value stored in app_meta.admin_password.
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let digest = hasher.finalize();
    format!("{:x}", digest)
}

#[derive(Debug, Serialize)]
struct TokenClaims {
    sub: String,
    iss: String,
    aud: String,
    exp: usize,
    iat: usize,
}

fn generate_token(app_state: &AppState, user_credential: &UserCredential) -> Result<String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("failed to get unix timestamp")?
        .as_secs() as usize;
    let exp = (SystemTime::now() + Duration::from_secs(24 * 60 * 60))
        .duration_since(UNIX_EPOCH)
        .context("failed to compute token expiration")?
        .as_secs() as usize;

    let claims = TokenClaims {
        sub: user_credential.username.clone(),
        iss: app_state.jwt_validator.issuer.clone(),
        aud: app_state.jwt_validator.audience.clone(),
        exp,
        iat: now,
    };

    let private_key_pem = app_state
        .env_config
        .lock()
        .expect("failed to lock app config for token generation")
        .server
        .token_private_key
        .clone();

    let key = EncodingKey::from_rsa_pem(private_key_pem.as_bytes())
        .context("failed to parse rsa private key pem")?;
    let header = Header::new(Algorithm::RS256);

    encode(&header, &claims, &key).context("failed to encode jwt")
}