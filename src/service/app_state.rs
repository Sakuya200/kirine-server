use std::sync::{Arc, Mutex};
use crate::config::EnvConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env_config: Arc<Mutex<EnvConfig>>
}

impl AppState {
    pub fn new(env_config: EnvConfig) -> AppState {
        AppState {
            env_config: Arc::new(Mutex::new(env_config))
        }
    }

}