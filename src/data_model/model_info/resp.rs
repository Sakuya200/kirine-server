use crate::data_model::model_info::types::ModelDownloadType;
use crate::pipeline::{BaseModel, HardwareType};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModelInfoResponse {
    pub id: i64,
    pub base_model: BaseModel,
    pub model_name: String,
    pub model_version: String,
    pub download_type: ModelDownloadType,
    pub required_model_name_list: Vec<String>,
    pub required_model_repo_id_list: Vec<String>,
    pub supported_feature_list: Vec<String>,
    pub supported_devices: Vec<HardwareType>,
    pub downloaded: bool,
    pub create_time: OffsetDateTime,
    pub modify_time: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModelMutationResult {
    pub model: ModelInfoResponse,
    pub removed_paths: Vec<String>,
    pub preserved_paths: Vec<String>,
}
