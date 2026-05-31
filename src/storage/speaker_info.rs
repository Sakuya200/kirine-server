use crate::data_models::speaker::{
    CreateSpeakerRequest, ImportModelAsSpeakerRequest, SpeakerInfoResponse, UpdateSpeakerRequest,
};
use crate::data_models::{AppLanguage, SpeakerSource, SpeakerStatus};
use crate::storage::LocalStorage;
use crate::storage::entity::{model_info, speaker};
use crate::utils::{current_native_time, from_native_to_offset_time};
use anyhow::{Result, bail};
use async_trait::async_trait;
use sea_orm::prelude::Expr;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, NotSet, QueryFilter, QueryOrder, Set};
use std::io;
use std::path::PathBuf;

#[async_trait]
pub trait SpeakerInfoStorage {
    async fn create_speaker_info(
        &self,
        request: CreateSpeakerRequest,
    ) -> Result<SpeakerInfoResponse>;

    async fn import_model_as_speaker(
        &self,
        request: ImportModelAsSpeakerRequest,
    ) -> Result<SpeakerInfoResponse>;

    async fn list_speaker_infos(&self) -> Result<Vec<SpeakerInfoResponse>>;

    async fn update_speaker_info(
        &self,
        request: UpdateSpeakerRequest,
    ) -> Result<SpeakerInfoResponse>;

    async fn delete_speaker_info(&self, speaker_id: i64) -> Result<bool>;
}

#[async_trait]
impl SpeakerInfoStorage for LocalStorage {
    async fn create_speaker_info(
        &self,
        request: CreateSpeakerRequest,
    ) -> Result<SpeakerInfoResponse> {
        let create_time = current_native_time();
        let languages = if request.languages.is_empty() {
            vec![AppLanguage::Chinese]
        } else {
            request.languages
        };
        let languages_json = serde_json::to_string(&languages)?;
        let name = request.name.trim();
        let description = request.description.trim();
        let status = request.status;
        let source = request.source;

        let inserted = speaker::ActiveModel {
            id: NotSet,
            name: Set(name.to_string()),
            languages_json: Set(languages_json),
            samples: Set(request.samples as i64),
            base_model: Set(request.base_model.as_str().to_string()),
            description: Set(description.to_string()),
            status: Set(status.as_str().to_string()),
            source: Set(source.as_str().to_string()),
            create_time: Set(create_time.clone()),
            modify_time: Set(create_time.clone()),
            deleted: Set(0),
        }
        .insert(&self.db_conn)
        .await?;

        map_speaker_model(inserted)
    }

    async fn import_model_as_speaker(
        &self,
        request: ImportModelAsSpeakerRequest,
    ) -> Result<SpeakerInfoResponse> {
        let create_time = current_native_time();
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

        // Keep import behavior consistent with client local mode: model type/version must be supported.
        model_info::Entity::find()
            .filter(model_info::Column::Deleted.eq(0))
            .filter(model_info::Column::BaseModel.eq(base_model))
            .filter(model_info::Column::ModelVersion.eq(model_version))
            .one(&self.db_conn)
            .await?
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "当前服务不支持该模型类型或版本: {} {}",
                        base_model, model_version
                    ),
                )
            })?;

        let languages_json = serde_json::to_string(&vec![request.language])?;
        let inserted = speaker::ActiveModel {
            id: NotSet,
            name: Set(name.to_string()),
            languages_json: Set(languages_json),
            samples: Set(0),
            base_model: Set(base_model.to_string()),
            description: Set(description.to_string()),
            status: Set(SpeakerStatus::Ready.as_str().to_string()),
            source: Set(SpeakerSource::Local.as_str().to_string()),
            create_time: Set(create_time),
            modify_time: Set(create_time),
            deleted: Set(0),
        }
        .insert(&self.db_conn)
        .await?;

        map_speaker_model(inserted)
    }

    async fn list_speaker_infos(&self) -> Result<Vec<SpeakerInfoResponse>> {
        speaker::Entity::find()
            .filter(speaker::Column::Deleted.eq(0))
            .order_by_desc(speaker::Column::ModifyTime)
            .order_by_desc(speaker::Column::CreateTime)
            .all(&self.db_conn)
            .await?
            .into_iter()
            .map(map_speaker_model)
            .collect()
    }

    async fn update_speaker_info(
        &self,
        request: UpdateSpeakerRequest,
    ) -> Result<SpeakerInfoResponse> {
        let modify_time = current_native_time();
        let speaker = speaker::Entity::find_by_id(request.id)
            .filter(speaker::Column::Deleted.eq(0))
            .one(&self.db_conn)
            .await?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "未找到目标说话人"))?;

        let mut active_model: speaker::ActiveModel = speaker.into();
        active_model.name = Set(request.name.trim().to_string());
        active_model.description = Set(request.description.trim().to_string());
        active_model.modify_time = Set(modify_time);

        let updated = active_model.update(&self.db_conn).await?;
        map_speaker_model(updated)
    }

    async fn delete_speaker_info(&self, speaker_id: i64) -> Result<bool> {
        let modify_time = current_native_time();
        let result = speaker::Entity::update_many()
            .col_expr(speaker::Column::Deleted, Expr::value(1))
            .col_expr(speaker::Column::ModifyTime, Expr::value(modify_time))
            .filter(speaker::Column::Id.eq(speaker_id))
            .filter(speaker::Column::Deleted.eq(0))
            .exec(&self.db_conn)
            .await?;

        Ok(result.rows_affected > 0)
    }
}

fn map_speaker_model(model: speaker::Model) -> Result<SpeakerInfoResponse> {
    let languages = serde_json::from_str::<Vec<AppLanguage>>(&model.languages_json)?;
    Ok(SpeakerInfoResponse {
        id: model.id,
        name: model.name,
        languages,
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
