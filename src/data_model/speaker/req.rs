use crate::data_model::speaker::types::{SpeakerSource, SpeakerStatus};
use crate::pipeline::BaseModel;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSpeakerRequest {
    pub speaker_name: String,
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
    pub speaker_name: String,
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
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerListFilter {
    pub keyword: Option<String>,
    pub status: Option<SpeakerStatus>,
}
