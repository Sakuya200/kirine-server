use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeviceTypeRequest {
	pub base_model: String,
	pub model_version: String,
}
