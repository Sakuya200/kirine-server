use crate::pipeline::{AttentionImplementation, HardwareType};
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs;
use std::path::PathBuf;
use crate::utils::{config_file_path, root_dir_path};
use anyhow::Result;
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding};
use rsa::rand_core::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey};
use tracing::{info, warn};

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
    pub db_url: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    #[serde(skip)]
    pub token_public_key: String,
    #[serde(skip)]
    pub token_private_key: String,
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
        Self {
            port: 3000,
            db_url: "localhost:5432".to_string(),
            db_user: "postgres".to_string(),
            db_password: "123456".to_string(),
            db_name: "postgres".to_string(),
            token_public_key: "".to_string(),
            token_private_key: "".to_string(),
        }
    }
}

impl EnvConfig {
    /// 将配置写回到配置文件中
    pub fn flush_env_config(&self) -> Result<()> {
        let toml_path = config_file_path()?;
        let toml_str = toml::to_string_pretty(self)
            .map_err(|e| anyhow::anyhow!("Failed to serialize config to TOML string: {e}"))?;;
        std::fs::write(&toml_path, toml_str)
        .map_err(|e| anyhow::anyhow!("Failed to write config to TOML string: {e}"))?;
        Ok(())
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

    pub fn load_token_key(&mut self) {
        // 从工作目录下的 publickey.pem 和 privatekey.pem 加载；不存在时自动生成。
        let root = root_dir_path().expect("Failed to get current working directory for JWT keys");
        let public_key_path = root.join("publickey.pem");
        let private_key_path = root.join("privatekey.pem");

        if !private_key_path.exists() || !private_key_path.exists() {
            println!("JWT key files do not exist at {:?} and {:?}, they will be generated automatically", public_key_path, private_key_path);
            let mut rng = OsRng;
            let private = RsaPrivateKey::new(&mut rng, 2048)
                .expect("Failed to generate RSA private key");
            let public = RsaPublicKey::from(&private);

            let private_pem = private
                .to_pkcs8_pem(LineEnding::LF)
                .expect("Failed to encode RSA private key to PEM")
                .to_string();
            let public_pem = public
                .to_public_key_pem(LineEnding::LF)
                .expect("Failed to encode RSA public key to PEM");

            fs::write(&private_key_path, &private_pem)
                .unwrap_or_else(|e| panic!("Failed to write private key to {:?}: {e}", private_key_path));
            fs::write(&public_key_path, &public_pem)
                .unwrap_or_else(|e| panic!("Failed to write public key to {:?}: {e}", public_key_path));
            println!("Successfully generated JWT RSA key pair and saved to {:?} and {:?}", public_key_path, private_key_path);
        }

        let public_key = fs::read_to_string(&public_key_path).ok();
        let private_key = fs::read_to_string(&private_key_path).ok();

        self.server.token_public_key = public_key.expect("JWT public key failed to load from file");
        self.server.token_private_key = private_key.expect("JWT private key failed to load from file");
    }
}
