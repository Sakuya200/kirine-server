mod env_config;
mod log_config;
mod ui_config;

pub use env_config::EnvConfig;

pub fn init_config() -> EnvConfig {
    let env_config = EnvConfig::from_config_toml();
    // 将默认值写回配置文件
    env_config.flush_env_config();
    env_config
}