use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ModelDownloadType {
    #[serde(rename = "HF-Like")]
    HfLike,
    #[serde(rename = "Custom")]
    Custom,
}

impl ModelDownloadType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HfLike => "HF-Like",
            Self::Custom => "Custom",
        }
    }
}

impl fmt::Display for ModelDownloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ModelDownloadType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "HF-Like" | "hf-like" | "hflike" | "hf_like" => Ok(Self::HfLike),
            "Custom" | "custom" => Ok(Self::Custom),
            other => Err(format!("不支持的模型下载类型: {}", other)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelDownloadState {
    Downloaded,
    Uninstalled,
}

impl ModelDownloadState {
    pub const fn as_downloaded(self) -> bool {
        match self {
            Self::Downloaded => true,
            Self::Uninstalled => false,
        }
    }
}
