use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::pipeline::{AttentionImplementation, HardwareType};

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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case", default)]
pub struct BasicConfig {
    pub data_dir: Option<String>,
    pub log_dir: Option<String>,
    pub model_dir: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case", default)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case", default)]
pub struct TrainingConfig {
    pub hardware_type: HardwareType,
    pub attn_implementation: AttentionImplementation,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            hardware_type: HardwareType::default(),
            attn_implementation: AttentionImplementation::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 3000,
        }
    }
}

impl TrainingConfig {
    pub fn with_hardware_type(mut self, hardware_type: HardwareType) -> Self {
        self.hardware_type = hardware_type;
        self
    }

    pub fn with_attn_implementation(
        mut self,
        attn_implementation: AttentionImplementation,
    ) -> Self {
        self.attn_implementation = attn_implementation;
        self
    }
}


impl EnvConfig {

    /// 将配置写回到配置文件中
    pub fn flush_env_config(&self) -> Result<()> {
        Ok(())
    }

    pub fn from_config_toml(toml_path: &Path) -> EnvConfig {
        let toml_str = std::fs::read_to_string(toml_path).unwrap_or_else(|e| {
            panic!("Failed to read config file at {:?}: {e}", toml_path)
        });
        toml::from_str(&toml_str).unwrap_or_else(|e| {
            panic!("Failed to parse config file at {:?}: {e}", toml_path)
        })
    }
}