use crate::data_model::common::{AppLanguage, TextToSpeechFormat};
use crate::data_model::history::resp::ModelTrainingSampleInput;
use crate::pipeline::{BaseModel, HardwareType};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct CreateTextToSpeechTaskDto {
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

#[derive(Debug, Clone)]
pub struct CreateModelTrainingTaskDto {
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

#[derive(Debug, Clone)]
pub struct CreateVoiceCloneTaskDto {
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
