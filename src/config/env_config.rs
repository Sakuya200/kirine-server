use crate::pipeline::{AttentionImplementation, HardwareType};
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::path::PathBuf;
use crate::utils::config_file_path;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub struct EnvConfig {
    #[serde(default)]
    pub basic: BasicConfig,
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub training: TrainingConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case", default)]
pub struct BasicConfig {
    pub data_dir: String,
    pub log_dir: String,
    pub model_dir: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case", default)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case", default)]
pub struct TrainingConfig {
    pub global_device_type: HardwareType,
    pub attn_implementation: AttentionImplementation,
}

impl Default for BasicConfig {
    fn default() -> Self {
        Self {
            data_dir: "./data".to_string(),
            log_dir: "./logs".to_string(),
            model_dir: "./models".to_string(),
        }
    }
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            global_device_type: HardwareType::default(),
            attn_implementation: AttentionImplementation::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self { port: 3000 }
    }
}

impl EnvConfig {
    /// 将配置写回到配置文件中
    pub fn flush_env_config(&self) {
        let toml_path = current_dir()
            .expect("failed to get current directory")
            .join("config.toml");
        let toml_str = toml::to_string_pretty(self)
            .expect("Failed to serialize EnvConfig to TOML string");
        std::fs::write(&toml_path, toml_str)
            .unwrap_or_else(|e| panic!("Failed to write config file at {:?}: {e}", toml_path));
    }

    pub fn from_config_toml() -> EnvConfig {
        let toml_path = config_file_path()
            .unwrap_or_else(|e| panic!("Failed to read config file at {:?}: {e}", config_file_path()));
        if !toml_path.exists() {
            println!("config file does not exist: {:?}, app will create a new config file", toml_path);
            // 如果文件不存在则创建新文件
            std::fs::File::create(&toml_path)
                .expect("failed to create config.toml");
        }
        let toml_str = std::fs::read_to_string(&toml_path)
            .unwrap_or_else(|e| panic!("Failed to read config file at {:?}: {e}", toml_path));
        toml::from_str(&toml_str)
            .unwrap_or_else(|e| panic!("Failed to parse config file at {:?}: {e}", toml_path))
    }

    pub fn data_dir(&self) -> PathBuf { self.basic.data_dir.clone().into() }

    pub fn log_dir(&self) -> PathBuf {
        self.basic.log_dir.clone().into()
    }

    pub fn model_dir(&self) -> PathBuf {
        self.basic.model_dir.clone().into()
    }

    pub fn global_device_type(&self) -> HardwareType {
        self.training.global_device_type.clone()
    }

    pub fn attn_implementation(&self) -> AttentionImplementation {
        self.training.attn_implementation.clone()
    }
}
