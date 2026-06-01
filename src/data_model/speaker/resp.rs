use crate::data_model::common::AppLanguage;
use crate::data_model::speaker::types::{SpeakerSource, SpeakerStatus};
use crate::pipeline::BaseModel;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

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
