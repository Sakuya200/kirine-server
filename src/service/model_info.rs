use anyhow::Result;
use async_trait::async_trait;
use std::io;

use crate::api::entity::{PageRequest, PageResponse};
use crate::data_model::model_info::req::ModelListFilter;
use crate::data_model::model_info::resp::{ModelInfoResponse, ModelMutationResult};
use crate::data_model::model_info::types::{ModelDownloadState, ModelDownloadType};
use crate::pipeline::HardwareType;
use crate::service::AppState;
use crate::storage::{ModelInfoStorage, model_info_entity as model_info};
use crate::utils::from_native_to_offset_time;

#[async_trait]
pub trait ModelInfoService {
    async fn list_model_infos(
        &self,
        request: PageRequest<ModelListFilter>,
    ) -> Result<PageResponse<ModelInfoResponse>>;
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
    async fn list_model_infos(
        &self,
        request: PageRequest<ModelListFilter>,
    ) -> Result<PageResponse<ModelInfoResponse>> {
        let PageRequest {
            page,
            page_size,
            filter,
        } = request;

        let mut rows = self.storage.list_model_info_entities().await?;
        if let Some(keyword) = filter.keyword.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
            rows.retain(|row| {
                row.base_model.contains(keyword)
                    || row.model_name.contains(keyword)
                    || row.model_version.contains(keyword)
            });
        }
        if let Some(downloaded) = filter.downloaded {
            rows.retain(|row| row.downloaded == downloaded);
        }
        if let Some(feature) = filter.feature {
            rows.retain(|row| {
                let supported = serde_json::from_str::<Vec<String>>(&row.supported_feature_list_json)
                    .unwrap_or_default();
                supported.iter().any(|item| item == feature.as_str())
            });
        }

        let total = rows.len() as u64;
        let page = page.max(1);
        let page_size = page_size.max(1);
        let start = ((page - 1) * page_size) as usize;
        let items = rows
            .into_iter()
            .skip(start)
            .take(page_size as usize)
            .map(map_model_info_entity)
            .collect::<Result<Vec<_>>>()?;

        Ok(PageResponse::new(total, page, page_size, items))
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
        supported_languages: serde_json::from_str(&row.supported_languages)?,
        downloaded: row.downloaded,
        create_time: from_native_to_offset_time(row.create_time),
        modify_time: from_native_to_offset_time(row.modify_time),
    })
}
