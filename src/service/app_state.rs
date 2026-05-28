use std::sync::{Arc, Mutex};
use crate::config::EnvConfig;
use crate::storage::LocalStorage;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env_config: Arc<Mutex<EnvConfig>>,
    pub storage: LocalStorage,
}

impl AppState {
    pub fn new(env_config: EnvConfig, storage: LocalStorage) -> AppState {
        AppState {
            env_config: Arc::new(Mutex::new(env_config)),
            storage,
        }
    }

}