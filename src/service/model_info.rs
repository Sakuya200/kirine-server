use anyhow::Result;
use async_trait::async_trait;
use std::io;

use crate::data_model::model_info::resp::{ModelInfoResponse, ModelMutationResult};
use crate::data_model::model_info::types::{ModelDownloadState, ModelDownloadType};
use crate::pipeline::HardwareType;
use crate::service::AppState;
use crate::storage::{ModelInfoStorage, model_info_entity as model_info};
use crate::utils::from_native_to_offset_time;

#[async_trait]
pub trait ModelInfoService {
    async fn list_model_infos(&self) -> Result<Vec<ModelInfoResponse>>;
    async fn get_model_info(&self, model_id: i64) -> Result<ModelInfoResponse>;
    async fn get_device_type(&self, base_model: &str, model_version: &str) -> Result<HardwareType>;
    async fn install_model(
        &self,
        model_id: i64,
        device: HardwareType,
    ) -> Result<ModelMutationResult>;
    async fn uninstall_model(&self, model_id: i64) -> Result<ModelMutationResult>;
}

#[async_trait]
impl ModelInfoService for AppState {
    async fn list_model_infos(&self) -> Result<Vec<ModelInfoResponse>> {
        let rows = self.storage.list_model_info_entities().await?;
        rows.into_iter().map(map_model_info_entity).collect()
    }

    async fn get_model_info(&self, model_id: i64) -> Result<ModelInfoResponse> {
        let row = self.storage.get_model_info_entity(model_id).await?;
        map_model_info_entity(row)
    }

    async fn get_device_type(&self, base_model: &str, model_version: &str) -> Result<HardwareType> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            format!(
                "设备探测依赖运行时命令调用，当前仅保留接口占位: {} {}",
                base_model, model_version
            ),
        )
        .into())
    }

    async fn install_model(
        &self,
        model_id: i64,
        device: HardwareType,
    ) -> Result<ModelMutationResult> {
        let _ = device;
        let row = self
            .storage
            .update_model_download_state(model_id, ModelDownloadState::Downloaded)
            .await?;
        Ok(ModelMutationResult {
            model: map_model_info_entity(row)?,
            removed_paths: Vec::new(),
            preserved_paths: Vec::new(),
        })
    }

    async fn uninstall_model(&self, model_id: i64) -> Result<ModelMutationResult> {
        let row = self
            .storage
            .update_model_download_state(model_id, ModelDownloadState::Uninstalled)
            .await?;
        Ok(ModelMutationResult {
            model: map_model_info_entity(row)?,
            removed_paths: Vec::new(),
            preserved_paths: Vec::new(),
        })
    }
}

fn map_model_info_entity(row: model_info::Model) -> Result<ModelInfoResponse> {
    Ok(ModelInfoResponse {
        id: row.id,
        base_model: row.base_model,
        model_name: row.model_name,
        model_version: row.model_version,
        download_type: row
            .download_type
            .parse::<ModelDownloadType>()
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?,
        required_model_name_list: serde_json::from_str(&row.required_model_name_list_json)?,
        required_model_repo_id_list: serde_json::from_str(&row.required_model_repo_id_list_json)?,
        supported_feature_list: serde_json::from_str(&row.supported_feature_list_json)?,
        supported_devices: serde_json::from_str(&row.supported_devices)?,
        downloaded: row.downloaded,
        create_time: from_native_to_offset_time(row.create_time),
        modify_time: from_native_to_offset_time(row.modify_time),
    })
}
