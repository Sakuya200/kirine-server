use crate::data_model::speaker::types::{SpeakerSource, SpeakerStatus};
use crate::pipeline::BaseModel;

#[derive(Debug, Clone)]
pub struct CreateSpeakerDto {
    pub name: String,
    pub samples: u32,
    pub base_model: BaseModel,
    pub description: String,
    pub status: SpeakerStatus,
    pub source: SpeakerSource,
}

#[derive(Debug, Clone)]
pub struct ImportModelAsSpeakerDto {
    pub base_model: BaseModel,
    pub model_version: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct UpdateSpeakerDto {
    pub id: i64,
    pub name: String,
    pub description: String,
}
