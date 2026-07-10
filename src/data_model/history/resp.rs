use crate::data_model::common::{AppLanguage, TextToSpeechFormat};
use crate::data_model::history::types::{
    HistoryTaskType, ModelTrainingFileKind, ModelTrainingSampleType, TaskStatus,
};
use crate::pipeline::{BaseModel, HardwareType};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModelTrainingFileInput {
    pub file_name: String,
    pub file_kind: ModelTrainingFileKind,
    pub file_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModelTrainingSampleInput {
    pub id: i64,
    pub sample_type: ModelTrainingSampleType,
    pub title: String,
    pub detail: String,
    pub transcript_preview: Option<String>,
    pub primary_file: ModelTrainingFileInput,
    pub secondary_file: Option<ModelTrainingFileInput>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextToSpeechTaskDetail {
    pub speaker_id: Option<i64>,
    pub base_model: BaseModel,
    pub model_version: String,
    pub language: AppLanguage,
    pub format: TextToSpeechFormat,
    pub export_audio_name: String,
    pub text: String,
    pub model_params: Value,
    pub char_count: usize,
    pub file_name: String,
    pub output_file_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModelTrainingTaskDetail {
    pub language: AppLanguage,
    pub base_model: BaseModel,
    pub model_version: String,
    pub speaker_name: String,
    pub description: String,
    pub model_params: Value,
    pub sample_count: i64,
    pub samples: Vec<ModelTrainingSampleInput>,
    pub notes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VoiceCloneTaskDetail {
    pub base_model: BaseModel,
    pub model_version: String,
    pub language: AppLanguage,
    pub format: TextToSpeechFormat,
    pub export_audio_name: String,
    pub ref_audio_name: String,
    pub ref_audio_path: String,
    pub ref_text: String,
    pub text: String,
    pub model_params: Value,
    pub char_count: usize,
    pub file_name: String,
    pub output_file_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HistoryRecordResponse {
    pub id: i64,
    pub task_type: HistoryTaskType,
    pub title: String,
    pub speaker: String,
    pub status: TaskStatus,
    pub duration_seconds: i64,
    pub device: HardwareType,
    pub create_time: OffsetDateTime,
    pub modify_time: OffsetDateTime,
    pub task_log: Option<String>,
    pub detail: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HistoryRecordSummaryResponse {
    pub id: i64,
    pub task_type: HistoryTaskType,
    pub title: String,
    pub speaker: String,
    pub status: TaskStatus,
    pub duration_seconds: i64,
    pub device: HardwareType,
    pub create_time: OffsetDateTime,
    pub modify_time: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextToSpeechAudioAsset {
    pub task_id: i64,
    pub file_name: String,
    pub content_type: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VoiceCloneAudioAsset {
    pub task_id: i64,
    pub file_name: String,
    pub content_type: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VoiceDesignAudioAsset {
    pub task_id: i64,
    pub file_name: String,
    pub content_type: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VoiceDesignTaskResult {
    pub task_id: i64,
    pub file_name: String,
    pub base_model: BaseModel,
    pub model_version: String,
    pub language: AppLanguage,
    pub format: TextToSpeechFormat,
    pub export_audio_name: String,
    pub duration_seconds: i64,
    pub prompt: String,
    pub text: String,
    pub model_params: Value,
    pub created_at: OffsetDateTime,
    pub status: TaskStatus,
    pub output_file_path: String,
}
