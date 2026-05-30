use std::sync::{Arc, Mutex};
use crate::api::auth::{JwtParser};
use crate::config::EnvConfig;
use crate::storage::LocalStorage;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env_config: Arc<Mutex<EnvConfig>>,
    pub storage: LocalStorage,
    pub jwt_validator: JwtParser,
}

impl AppState {
    pub fn new(env_config: EnvConfig, storage: LocalStorage) -> AppState {
        let jwt_validator = JwtParser::new(env_config.server.token_public_key.clone());
        AppState {
            env_config: Arc::new(Mutex::new(env_config)),
            storage,
            jwt_validator
        }
    }

}