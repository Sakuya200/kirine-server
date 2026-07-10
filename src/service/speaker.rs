use crate::data_model::speaker::dto::{
    CreateSpeakerDto, ImportModelAsSpeakerDto, UpdateSpeakerDto,
};
use crate::data_model::speaker::req::{
    CreateSpeakerRequest, ImportModelAsSpeakerRequest, SpeakerListFilter, UpdateSpeakerRequest,
};
use crate::data_model::speaker::resp::SpeakerInfoResponse;
use crate::api::entity::{PageRequest, PageResponse};
use crate::data_model::speaker::types::{
    SpeakerDeletedState, SpeakerSource, SpeakerStatus,
};
use crate::service::AppState;
use crate::storage::{SpeakerInfoStorage, speaker_entity as speaker};
use crate::utils::from_native_to_offset_time;
use anyhow::{Result, bail};
use async_trait::async_trait;
use std::io;
use std::path::PathBuf;

#[async_trait]
pub trait SpeakerService {
    async fn create_speaker_info(
        &self,
        request: CreateSpeakerRequest,
    ) -> Result<SpeakerInfoResponse>;

    async fn import_model_as_speaker(
        &self,
        request: ImportModelAsSpeakerRequest,
    ) -> Result<SpeakerInfoResponse>;

    async fn list_speaker_infos(
        &self,
        request: PageRequest<SpeakerListFilter>,
    ) -> Result<PageResponse<SpeakerInfoResponse>>;

    async fn update_speaker_info(
        &self,
        request: UpdateSpeakerRequest,
    ) -> Result<SpeakerInfoResponse>;

    async fn delete_speaker_info(&self, speaker_id: i64) -> Result<bool>;
}

#[async_trait]
impl SpeakerService for AppState {
    async fn create_speaker_info(
        &self,
        request: CreateSpeakerRequest,
    ) -> Result<SpeakerInfoResponse> {
        let row = self
            .storage
            .create_speaker_info(CreateSpeakerDto {
                name: request.speaker_name,
                samples: request.samples,
                base_model: request.base_model,
                description: request.description,
                status: request.status,
                source: request.source,
            })
            .await?;
        map_speaker_entity(row)
    }

    async fn import_model_as_speaker(
        &self,
        request: ImportModelAsSpeakerRequest,
    ) -> Result<SpeakerInfoResponse> {
        let name = request.name.trim();
        let description = request.description.trim();
        let base_model = request.base_model.trim();
        let model_version = request.model_version.trim();
        let source_model_dir = PathBuf::from(request.source_model_dir_path.trim());

        if name.is_empty() {
            bail!("说话人名称不能为空");
        }
        if description.is_empty() {
            bail!("说话人描述不能为空");
        }
        if base_model.is_empty() {
            bail!("基础模型类型不能为空");
        }
        if model_version.is_empty() {
            bail!("模型版本不能为空");
        }
        if !source_model_dir.is_dir() {
            bail!("模型目录不存在或不是目录: {}", source_model_dir.display());
        }

        let row = self
            .storage
            .import_model_as_speaker(ImportModelAsSpeakerDto {
                base_model: request.base_model,
                model_version: request.model_version,
                name: request.name,
                description: request.description,
            })
            .await?;
        map_speaker_entity(row)
    }

    async fn list_speaker_infos(
        &self,
        request: PageRequest<SpeakerListFilter>,
    ) -> Result<PageResponse<SpeakerInfoResponse>> {
        let PageRequest {
            page,
            page_size,
            filter,
        } = request;

        let mut rows = self.storage.list_speaker_entities().await?;
        if let Some(keyword) = filter.keyword.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
            rows.retain(|row| row.name.contains(keyword) || row.description.contains(keyword));
        }
        if let Some(status) = filter.status {
            let status = status.as_str();
            rows.retain(|row| row.status == status);
        }

        let total = rows.len() as u64;
        let page = page.max(1);
        let page_size = page_size.max(1);
        let start = ((page - 1) * page_size) as usize;
        let items = rows
            .into_iter()
            .skip(start)
            .take(page_size as usize)
            .map(map_speaker_entity)
            .collect::<Result<Vec<_>>>()?;

        Ok(PageResponse::new(total, page, page_size, items))
    }

    async fn update_speaker_info(
        &self,
        request: UpdateSpeakerRequest,
    ) -> Result<SpeakerInfoResponse> {
        let row = self
            .storage
            .update_speaker_info(UpdateSpeakerDto {
                id: request.id,
                name: request.speaker_name,
                description: request.description,
            })
            .await?;
        map_speaker_entity(row)
    }

    async fn delete_speaker_info(&self, speaker_id: i64) -> Result<bool> {
        self.storage
            .update_speaker_deleted_state(speaker_id, SpeakerDeletedState::Deleted)
            .await
    }
}

fn map_speaker_entity(model: speaker::Model) -> Result<SpeakerInfoResponse> {
    Ok(SpeakerInfoResponse {
        id: model.id,
        speaker_name: model.name,
        samples: model.samples as u32,
        base_model: model.base_model,
        create_time: from_native_to_offset_time(model.create_time),
        modify_time: from_native_to_offset_time(model.modify_time),
        description: model.description,
        status: model
            .status
            .parse::<SpeakerStatus>()
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?,
        source: model
            .source
            .parse::<SpeakerSource>()
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?,
    })
}
