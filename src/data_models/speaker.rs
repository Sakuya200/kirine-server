use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use crate::data_models::{AppLanguage, SpeakerSource, SpeakerStatus};
use crate::pipeline::BaseModel;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerInfoResponse {
    pub id: i64,
    pub name: String,
    pub languages: Vec<AppLanguage>,
    pub samples: u32,
    pub base_model: BaseModel,
    pub create_time: OffsetDateTime,
    pub modify_time: OffsetDateTime,
    pub description: String,
    pub status: SpeakerStatus,
    pub source: SpeakerSource,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSpeakerRequest {
    pub name: String,
    pub languages: Vec<AppLanguage>,
    pub samples: u32,
    pub base_model: BaseModel,
    pub description: String,
    pub status: SpeakerStatus,
    pub source: SpeakerSource,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSpeakerRequest {
    pub id: i64,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportModelAsSpeakerRequest {
    pub base_model: BaseModel,
    pub model_version: String,
    pub source_model_dir_path: String,
    pub name: String,
    pub description: String,
    pub language: AppLanguage,
}