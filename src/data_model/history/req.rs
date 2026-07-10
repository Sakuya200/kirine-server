use crate::data_model::common::{AppLanguage, TextToSpeechFormat};
use crate::pipeline::{BaseModel, HardwareType};
use serde::Deserialize;
use serde_json::Value;

use crate::data_model::history::resp::ModelTrainingSampleInput;
use crate::data_model::history::types::{HistoryTaskType, TaskStatus};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTextToSpeechTaskRequest {
	pub title: String,
	pub speaker_id: Option<i64>,
	pub speaker_name: String,
	pub base_model: BaseModel,
	pub model_version: String,
	pub language: AppLanguage,
	pub format: TextToSpeechFormat,
	pub export_audio_name: String,
	pub text: String,
	pub model_params: Value,
	pub device: HardwareType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateModelTrainingTaskRequest {
	pub title: String,
	pub speaker_name: String,
	pub language: AppLanguage,
	pub base_model: BaseModel,
	pub model_version: String,
	pub description: String,
	pub model_params: Value,
	pub samples: Vec<ModelTrainingSampleInput>,
	pub notes: Vec<String>,
	pub device: HardwareType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVoiceCloneTaskRequest {
	pub title: String,
	pub speaker_name: String,
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
	pub device: HardwareType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVoiceDesignTaskRequest {
	pub title: String,
	pub speaker_name: String,
	pub base_model: BaseModel,
	pub model_version: String,
	pub language: AppLanguage,
	pub format: TextToSpeechFormat,
	pub export_audio_name: String,
	pub prompt: String,
	pub text: String,
	pub model_params: Value,
	pub device: HardwareType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskStatusRequest {
	pub task_id: i64,
	pub status: TaskStatus,
	pub duration_seconds: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryListFilter {
	pub keyword: Option<String>,
	pub task_type: Option<HistoryTaskType>,
	pub status: Option<TaskStatus>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteHistoryQuery {
	pub task_type: HistoryTaskType,
}
