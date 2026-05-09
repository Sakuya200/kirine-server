mod env_config;
mod log_config;
mod ui_config;

use std::env::current_dir;
pub use env_config::EnvConfig;

pub fn init_config() -> EnvConfig {
    let cur_dir = current_dir().expect("failed to get current directory");
    EnvConfig::from_config_toml(cur_dir.as_path())
}