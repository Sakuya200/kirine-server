use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

pub(crate) mod speaker;


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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum HistoryTaskType {
    ModelTraining,
    TextToSpeech,
    VoiceClone,
}

impl HistoryTaskType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModelTraining => "model-training",
            Self::TextToSpeech => "text-to-speech",
            Self::VoiceClone => "voice-clone",
        }
    }

    pub const fn storage_dir(self) -> &'static str {
        match self {
            Self::ModelTraining => "model_training",
            Self::TextToSpeech => "tts",
            Self::VoiceClone => "voice_clone",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Cancelled,
    Failed,
}

impl TaskStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
        }
    }

    pub const fn is_finished(self) -> bool {
        matches!(self, Self::Completed | Self::Cancelled | Self::Failed)
    }
}

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

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for TaskStatus {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "pending" => Ok(Self::Pending),
            "running" => Ok(Self::Running),
            "completed" => Ok(Self::Completed),
            "cancelled" => Ok(Self::Cancelled),
            "failed" => Ok(Self::Failed),
            other => Err(format!("不支持的任务状态: {}", other)),
        }
    }
}

impl fmt::Display for HistoryTaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for HistoryTaskType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "model-training" => Ok(Self::ModelTraining),
            "text-to-speech" => Ok(Self::TextToSpeech),
            "voice-clone" => Ok(Self::VoiceClone),
            other => Err(format!("不支持的历史任务类型: {}", other)),
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ModelTrainingSampleType {
    Single,
    Dataset,
}

impl ModelTrainingSampleType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Dataset => "dataset",
        }
    }
}

impl fmt::Display for ModelTrainingSampleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ModelTrainingSampleType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "single" => Ok(Self::Single),
            "dataset" => Ok(Self::Dataset),
            other => Err(format!("不支持的模型训练样本类型: {}", other)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ModelTrainingFileKind {
    Audio,
    Archive,
    Annotation,
}

impl ModelTrainingFileKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Audio => "audio",
            Self::Archive => "archive",
            Self::Annotation => "annotation",
        }
    }
}

impl fmt::Display for ModelTrainingFileKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ModelTrainingFileKind {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "audio" => Ok(Self::Audio),
            "archive" => Ok(Self::Archive),
            "annotation" => Ok(Self::Annotation),
            other => Err(format!("不支持的文件类型: {}", other)),
        }
    }
}