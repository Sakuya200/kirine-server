use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryRecordStateMutation {
    SoftDelete,
    Cancel,
}
