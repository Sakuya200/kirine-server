use crate::data_model::common::AppLanguage;
use crate::data_model::speaker::dto::{
    CreateSpeakerDto, ImportModelAsSpeakerDto, UpdateSpeakerDto,
};
use crate::data_model::speaker::types::{SpeakerDeletedState, SpeakerSource, SpeakerStatus};
use crate::storage::LocalStorage;
use crate::storage::entity::{model_info, speaker};
use crate::utils::current_native_time;
use anyhow::Result;
use async_trait::async_trait;
use sea_orm::prelude::Expr;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, NotSet, QueryFilter, QueryOrder, Set};
use std::io;

#[async_trait]
pub trait SpeakerInfoStorage {
    async fn create_speaker_info(&self, request: CreateSpeakerDto) -> Result<speaker::Model>;

    async fn import_model_as_speaker(
        &self,
        request: ImportModelAsSpeakerDto,
    ) -> Result<speaker::Model>;

    async fn list_speaker_entities(&self) -> Result<Vec<speaker::Model>>;

    async fn update_speaker_info(&self, request: UpdateSpeakerDto) -> Result<speaker::Model>;

    async fn update_speaker_deleted_state(
        &self,
        speaker_id: i64,
        state: SpeakerDeletedState,
    ) -> Result<bool>;
}

#[async_trait]
impl SpeakerInfoStorage for LocalStorage {
    async fn create_speaker_info(&self, request: CreateSpeakerDto) -> Result<speaker::Model> {
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

        speaker::ActiveModel {
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
        .await
        .map_err(Into::into)
    }

    async fn import_model_as_speaker(
        &self,
        request: ImportModelAsSpeakerDto,
    ) -> Result<speaker::Model> {
        let create_time = current_native_time();
        let name = request.name.trim();
        let description = request.description.trim();
        let base_model = request.base_model.trim();
        let model_version = request.model_version.trim();

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
        speaker::ActiveModel {
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
        .await
        .map_err(Into::into)
    }

    async fn list_speaker_entities(&self) -> Result<Vec<speaker::Model>> {
        speaker::Entity::find()
            .filter(speaker::Column::Deleted.eq(0))
            .order_by_desc(speaker::Column::ModifyTime)
            .order_by_desc(speaker::Column::CreateTime)
            .all(&self.db_conn)
            .await
            .map_err(Into::into)
    }

    async fn update_speaker_info(&self, request: UpdateSpeakerDto) -> Result<speaker::Model> {
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

        active_model.update(&self.db_conn).await.map_err(Into::into)
    }

    async fn update_speaker_deleted_state(
        &self,
        speaker_id: i64,
        state: SpeakerDeletedState,
    ) -> Result<bool> {
        let modify_time = current_native_time();
        let result = speaker::Entity::update_many()
            .col_expr(
                speaker::Column::Deleted,
                Expr::value(state.as_deleted_flag()),
            )
            .col_expr(speaker::Column::ModifyTime, Expr::value(modify_time))
            .filter(speaker::Column::Id.eq(speaker_id))
            .filter(speaker::Column::Deleted.eq(state.expected_current_flag()))
            .exec(&self.db_conn)
            .await?;

        Ok(result.rows_affected > 0)
    }
}
