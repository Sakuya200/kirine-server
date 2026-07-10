use crate::data_model::history::types::HistoryTaskType;
use crate::pipeline::HardwareType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeviceTypeRequest {
	pub base_model: String,
	pub model_version: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelListFilter {
	pub keyword: Option<String>,
	pub downloaded: Option<bool>,
	pub feature: Option<HistoryTaskType>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallModelQuery {
	pub device: HardwareType,
}
