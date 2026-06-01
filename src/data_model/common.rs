use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum AppLanguage {
    #[serde(rename = "chinese", alias = "zh-CN")]
    Chinese,
    #[serde(rename = "english", alias = "en-US")]
    English,
    #[serde(rename = "japanese", alias = "ja-JP")]
    Japanese,
}

impl AppLanguage {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Chinese => "chinese",
            Self::English => "english",
            Self::Japanese => "japanese",
        }
    }
}

impl fmt::Display for AppLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for AppLanguage {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "chinese" | "zh-CN" => Ok(Self::Chinese),
            "english" | "en-US" => Ok(Self::English),
            "japanese" | "ja-JP" => Ok(Self::Japanese),
            other => Err(format!("不支持的语言类型: {}", other)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TextToSpeechFormat {
    Wav,
    Mp3,
    Flac,
}

impl TextToSpeechFormat {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Wav => "wav",
            Self::Mp3 => "mp3",
            Self::Flac => "flac",
        }
    }
}

impl fmt::Display for TextToSpeechFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for TextToSpeechFormat {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "wav" => Ok(Self::Wav),
            "mp3" => Ok(Self::Mp3),
            "flac" => Ok(Self::Flac),
            other => Err(format!("不支持的音频格式: {}", other)),
        }
    }
}
