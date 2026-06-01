use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SpeakerStatus {
    Ready,
    Training,
    Disabled,
}

impl SpeakerStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Training => "training",
            Self::Disabled => "disabled",
        }
    }
}

impl fmt::Display for SpeakerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for SpeakerStatus {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "ready" => Ok(Self::Ready),
            "training" => Ok(Self::Training),
            "disabled" => Ok(Self::Disabled),
            other => Err(format!("不支持的说话人状态: {}", other)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SpeakerSource {
    Local,
    Preset,
    Remote,
}

impl SpeakerSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Preset => "preset",
            Self::Remote => "remote",
        }
    }
}

impl fmt::Display for SpeakerSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for SpeakerSource {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "local" => Ok(Self::Local),
            "preset" => Ok(Self::Preset),
            "remote" => Ok(Self::Remote),
            other => Err(format!("不支持的说话人来源: {}", other)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeakerDeletedState {
    Deleted,
}

impl SpeakerDeletedState {
    pub const fn as_deleted_flag(self) -> i32 {
        match self {
            Self::Deleted => 1,
        }
    }

    pub const fn expected_current_flag(self) -> i32 {
        match self {
            Self::Deleted => 0,
        }
    }
}
