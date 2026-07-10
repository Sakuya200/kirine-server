use std::io;

use anyhow::Result;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, NotSet, QueryFilter, QueryOrder,
    TransactionTrait, sea_query::Expr,
};

use crate::data_model::history::dto::{
    CreateModelTrainingTaskDto, CreateTextToSpeechTaskDto, CreateVoiceCloneTaskDto,
};
use crate::data_model::history::types::{HistoryRecordStateMutation, HistoryTaskType, TaskStatus};
use crate::storage::LocalStorage;
use crate::storage::entity::{task_history, training_task, tts_task, voice_clone_task};
use crate::utils::current_native_time;

pub(crate) enum HistoryDetailEntity {
    TextToSpeech(tts_task::Model),
    ModelTraining(training_task::Model),
    VoiceClone(voice_clone_task::Model),
}

pub(crate) struct HistoryRecordEntity {
    pub history: task_history::Model,
    pub detail: HistoryDetailEntity,
}

#[async_trait]
pub trait HistoryStorage {
    async fn create_text_to_speech_task(
        &self,
        request: CreateTextToSpeechTaskDto,
    ) -> Result<HistoryRecordEntity>;
    async fn create_model_training_task(
        &self,
        request: CreateModelTrainingTaskDto,
    ) -> Result<HistoryRecordEntity>;
    async fn create_voice_clone_task(
        &self,
        request: CreateVoiceCloneTaskDto,
    ) -> Result<HistoryRecordEntity>;
    async fn list_history_record_entities(&self) -> Result<Vec<HistoryRecordEntity>>;
    async fn get_history_record_entity(&self, history_id: i64) -> Result<HistoryRecordEntity>;
    async fn mutate_history_record_state(
        &self,
        history_id: i64,
        mutation: HistoryRecordStateMutation,
    ) -> Result<bool>;
}

#[async_trait]
impl HistoryStorage for LocalStorage {
    async fn create_text_to_speech_task(
        &self,
        request: CreateTextToSpeechTaskDto,
    ) -> Result<HistoryRecordEntity> {
        let now = current_native_time();
        let db = &self.db_conn;
        let txn = db.begin().await?;

        let task_type = HistoryTaskType::TextToSpeech;
        let history = task_history::ActiveModel {
            id: NotSet,
            task_type: Set(task_type.as_str().to_string()),
            title: Set(request.title),
            speaker_id: Set(request.speaker_id),
            speaker_name: Set(request.speaker_name),
            status: Set(TaskStatus::Pending.as_str().to_string()),
            duration_seconds: Set(0),
            create_time: Set(now),
            modify_time: Set(now),
            finished_time: Set(None),
            device: Set(request.device.as_str().to_string()),
            deleted: Set(0),
        }
        .insert(&txn)
        .await?;

        let detail = tts_task::ActiveModel {
            id: NotSet,
            history_id: Set(history.id),
            speaker_id: Set(request.speaker_id),
            model_path: Set(None),
            base_model: Set(request.base_model),
            model_version: Set(request.model_version),
            language: Set(request.language.as_str().to_string()),
            format: Set(request.format.as_str().to_string()),
            export_audio_name: Set(request.export_audio_name),
            text: Set(request.text.clone()),
            model_params_json: Set(serde_json::to_string(&request.model_params)?),
            char_count: Set(request.text.chars().count() as i32),
            output_file_path: Set(None),
            create_time: Set(now),
            modify_time: Set(now),
            deleted: Set(0),
        }
        .insert(&txn)
        .await?;

        txn.commit().await?;
        Ok(HistoryRecordEntity {
            history,
            detail: HistoryDetailEntity::TextToSpeech(detail),
        })
    }

    async fn create_model_training_task(
        &self,
        request: CreateModelTrainingTaskDto,
    ) -> Result<HistoryRecordEntity> {
        let now = current_native_time();
        let db = &self.db_conn;
        let txn = db.begin().await?;

        let task_type = HistoryTaskType::ModelTraining;
        let history = task_history::ActiveModel {
            id: NotSet,
            task_type: Set(task_type.as_str().to_string()),
            title: Set(request.title),
            speaker_id: Set(None),
            speaker_name: Set(request.speaker_name.clone()),
            status: Set(TaskStatus::Pending.as_str().to_string()),
            duration_seconds: Set(0),
            create_time: Set(now),
            modify_time: Set(now),
            finished_time: Set(None),
            device: Set(request.device.as_str().to_string()),
            deleted: Set(0),
        }
        .insert(&txn)
        .await?;

        let detail = training_task::ActiveModel {
            id: NotSet,
            history_id: Set(history.id),
            language: Set(request.language.as_str().to_string()),
            base_model: Set(request.base_model),
            model_version: Set(request.model_version),
            speaker_name: Set(request.speaker_name),
            description: Set(request.description),
            model_params_json: Set(serde_json::to_string(&request.model_params)?),
            sample_count: Set(request.samples.len() as i64),
            samples_json: Set(serde_json::to_string(&request.samples)?),
            notes_json: Set(serde_json::to_string(&request.notes)?),
            output_speaker_id: Set(None),
            create_time: Set(now),
            modify_time: Set(now),
            deleted: Set(0),
        }
        .insert(&txn)
        .await?;

        txn.commit().await?;
        Ok(HistoryRecordEntity {
            history,
            detail: HistoryDetailEntity::ModelTraining(detail),
        })
    }

    async fn create_voice_clone_task(
        &self,
        request: CreateVoiceCloneTaskDto,
    ) -> Result<HistoryRecordEntity> {
        let now = current_native_time();
        let db = &self.db_conn;
        let txn = db.begin().await?;

        let task_type = HistoryTaskType::VoiceClone;
        let history = task_history::ActiveModel {
            id: NotSet,
            task_type: Set(task_type.as_str().to_string()),
            title: Set(request.title),
            speaker_id: Set(None),
            speaker_name: Set(request.speaker_name),
            status: Set(TaskStatus::Pending.as_str().to_string()),
            duration_seconds: Set(0),
            create_time: Set(now),
            modify_time: Set(now),
            finished_time: Set(None),
            device: Set(request.device.as_str().to_string()),
            deleted: Set(0),
        }
        .insert(&txn)
        .await?;

        let detail = voice_clone_task::ActiveModel {
            id: NotSet,
            history_id: Set(history.id),
            base_model: Set(request.base_model),
            model_version: Set(request.model_version),
            language: Set(request.language.as_str().to_string()),
            format: Set(request.format.as_str().to_string()),
            export_audio_name: Set(request.export_audio_name),
            ref_audio_name: Set(request.ref_audio_name),
            ref_audio_path: Set(request.ref_audio_path),
            ref_text: Set(request.ref_text),
            text: Set(request.text.clone()),
            model_params_json: Set(serde_json::to_string(&request.model_params)?),
            char_count: Set(request.text.chars().count() as i32),
            output_file_path: Set(None),
            create_time: Set(now),
            modify_time: Set(now),
            deleted: Set(0),
        }
        .insert(&txn)
        .await?;

        txn.commit().await?;
        Ok(HistoryRecordEntity {
            history,
            detail: HistoryDetailEntity::VoiceClone(detail),
        })
    }

    async fn list_history_record_entities(&self) -> Result<Vec<HistoryRecordEntity>> {
        let rows = task_history::Entity::find()
            .filter(task_history::Column::Deleted.eq(0))
            .order_by_desc(task_history::Column::CreateTime)
            .order_by_desc(task_history::Column::Id)
            .all(&self.db_conn)
            .await?;

        let mut records = Vec::with_capacity(rows.len());
        for row in rows {
            records.push(load_history_record_entity(self, row).await?);
        }

        Ok(records)
    }

    async fn get_history_record_entity(&self, history_id: i64) -> Result<HistoryRecordEntity> {
        let row = task_history::Entity::find_by_id(history_id)
            .filter(task_history::Column::Deleted.eq(0))
            .one(&self.db_conn)
            .await?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "未找到目标任务"))?;

        load_history_record_entity(self, row).await
    }

    async fn mutate_history_record_state(
        &self,
        history_id: i64,
        mutation: HistoryRecordStateMutation,
    ) -> Result<bool> {
        let modify_time = current_native_time();
        let row = task_history::Entity::find_by_id(history_id)
            .filter(task_history::Column::Deleted.eq(0))
            .one(&self.db_conn)
            .await?;

        match mutation {
            HistoryRecordStateMutation::SoftDelete => {
                let Some(row) = row else {
                    return Ok(false);
                };

                let task_type = parse_history_task_type(&row.task_type)?;
                let result = task_history::Entity::update_many()
                    .col_expr(task_history::Column::Deleted, Expr::value(1))
                    .col_expr(task_history::Column::ModifyTime, Expr::value(modify_time))
                    .filter(task_history::Column::Id.eq(history_id))
                    .filter(task_history::Column::Deleted.eq(0))
                    .exec(&self.db_conn)
                    .await?;

                match task_type {
                    HistoryTaskType::TextToSpeech => {
                        tts_task::Entity::update_many()
                            .col_expr(tts_task::Column::Deleted, Expr::value(1))
                            .col_expr(tts_task::Column::ModifyTime, Expr::value(modify_time))
                            .filter(tts_task::Column::HistoryId.eq(history_id))
                            .filter(tts_task::Column::Deleted.eq(0))
                            .exec(&self.db_conn)
                            .await?;
                    }
                    HistoryTaskType::ModelTraining => {
                        training_task::Entity::update_many()
                            .col_expr(training_task::Column::Deleted, Expr::value(1))
                            .col_expr(training_task::Column::ModifyTime, Expr::value(modify_time))
                            .filter(training_task::Column::HistoryId.eq(history_id))
                            .filter(training_task::Column::Deleted.eq(0))
                            .exec(&self.db_conn)
                            .await?;
                    }
                    HistoryTaskType::VoiceClone => {
                        voice_clone_task::Entity::update_many()
                            .col_expr(voice_clone_task::Column::Deleted, Expr::value(1))
                            .col_expr(
                                voice_clone_task::Column::ModifyTime,
                                Expr::value(modify_time),
                            )
                            .filter(voice_clone_task::Column::HistoryId.eq(history_id))
                            .filter(voice_clone_task::Column::Deleted.eq(0))
                            .exec(&self.db_conn)
                            .await?;
                    }
                }

                Ok(result.rows_affected > 0)
            }
            HistoryRecordStateMutation::Cancel => {
                let row =
                    row.ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "未找到目标任务"))?;

                let status = parse_task_status(&row.status)?;
                if status.is_finished() {
                    return Ok(false);
                }

                let mut active_model: task_history::ActiveModel = row.into();
                active_model.status = Set(TaskStatus::Cancelled.as_str().to_string());
                active_model.modify_time = Set(modify_time);
                active_model.finished_time = Set(Some(modify_time));
                active_model.update(&self.db_conn).await?;

                Ok(true)
            }
        }
    }
}

async fn load_history_record_entity(
    storage: &LocalStorage,
    row: task_history::Model,
) -> Result<HistoryRecordEntity> {
    let task_type = parse_history_task_type(&row.task_type)?;
    let detail = load_history_detail_entity(storage, row.id, task_type).await?;
    Ok(HistoryRecordEntity {
        history: row,
        detail,
    })
}

async fn load_history_detail_entity(
    storage: &LocalStorage,
    history_id: i64,
    task_type: HistoryTaskType,
) -> Result<HistoryDetailEntity> {
    match task_type {
        HistoryTaskType::TextToSpeech => load_tts_detail_entity(storage, history_id).await,
        HistoryTaskType::ModelTraining => {
            load_model_training_detail_entity(storage, history_id).await
        }
        HistoryTaskType::VoiceClone => load_voice_clone_detail_entity(storage, history_id).await,
    }
}

async fn load_tts_detail_entity(
    storage: &LocalStorage,
    history_id: i64,
) -> Result<HistoryDetailEntity> {
    let row = tts_task::Entity::find()
        .filter(tts_task::Column::HistoryId.eq(history_id))
        .filter(tts_task::Column::Deleted.eq(0))
        .one(&storage.db_conn)
        .await?
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "未找到 TTS 任务详情"))?;
    Ok(HistoryDetailEntity::TextToSpeech(row))
}

async fn load_model_training_detail_entity(
    storage: &LocalStorage,
    history_id: i64,
) -> Result<HistoryDetailEntity> {
    let row = training_task::Entity::find()
        .filter(training_task::Column::HistoryId.eq(history_id))
        .filter(training_task::Column::Deleted.eq(0))
        .one(&storage.db_conn)
        .await?
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "未找到模型训练任务详情"))?;
    Ok(HistoryDetailEntity::ModelTraining(row))
}

async fn load_voice_clone_detail_entity(
    storage: &LocalStorage,
    history_id: i64,
) -> Result<HistoryDetailEntity> {
    let row = voice_clone_task::Entity::find()
        .filter(voice_clone_task::Column::HistoryId.eq(history_id))
        .filter(voice_clone_task::Column::Deleted.eq(0))
        .one(&storage.db_conn)
        .await?
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "未找到声音克隆任务详情"))?;
    Ok(HistoryDetailEntity::VoiceClone(row))
}

fn parse_history_task_type(value: &str) -> Result<HistoryTaskType> {
    value
        .parse::<HistoryTaskType>()
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err).into())
}

fn parse_task_status(value: &str) -> Result<TaskStatus> {
    value
        .parse::<TaskStatus>()
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err).into())
}
