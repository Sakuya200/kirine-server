mod env_config;
mod log_config;
mod ui_config;

use std::fs;
use tracing::warn;
pub use env_config::EnvConfig;
pub use log_config::init_log;

pub fn init_config() -> EnvConfig {
    let env_config = EnvConfig::from_config_toml();
    // 检查几个配置路径是否存在，如果不存在需要重新创建并警告
    let log_path = env_config.log_dir();
    let model_path = env_config.model_dir();
    let data_path = env_config.data_dir();

    if !log_path.exists() {
        warn!("Log directory does not exist, creating it: {:?}", log_path);
        fs::create_dir_all(&log_path)
            .unwrap_or_else(|e| panic!("Failed to create log directory at {:?}: {e}", log_path));
    }
    if !model_path.exists() {
        warn!("Model directory does not exist, creating it: {:?}", model_path);
        fs::create_dir_all(&model_path)
        .unwrap_or_else(|e| panic!("Failed to create model directory at {:?}: {e}", model_path));
    }
    if !data_path.exists() {
        warn!("Data directory does not exist, creating it: {:?}", data_path);
        fs::create_dir_all(&data_path)
        .unwrap_or_else(|e| panic!("Failed to create data directory at {:?}: {e}", data_path));
    }

    // 将默认值写回配置文件
    if let Err(e) = env_config.flush_env_config() {
        panic!("Failed to reflush env config to file: {e}");
    }
    env_config
}