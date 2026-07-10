use anyhow::{Result, bail};
use async_trait::async_trait;
use serde_json::Value;
use std::io;

use crate::data_model::common::TextToSpeechFormat;
use crate::data_model::history::dto::{
    CreateModelTrainingTaskDto, CreateTextToSpeechTaskDto, CreateVoiceCloneTaskDto,
};
use crate::data_model::history::req::{
    CreateModelTrainingTaskRequest, CreateTextToSpeechTaskRequest, CreateVoiceCloneTaskRequest,
    CreateVoiceDesignTaskRequest, HistoryListFilter,
};
use crate::data_model::history::resp::{
    HistoryRecordResponse, HistoryRecordSummaryResponse,
    ModelTrainingSampleInput, ModelTrainingTaskDetail, TextToSpeechAudioAsset,
    TextToSpeechTaskDetail, VoiceCloneAudioAsset, VoiceCloneTaskDetail, VoiceDesignAudioAsset,
    VoiceDesignTaskResult,
};
use crate::data_model::history::types::{HistoryRecordStateMutation, HistoryTaskType, TaskStatus};
use crate::pipeline::HardwareType;
use crate::api::entity::{PageRequest, PageResponse};
use crate::service::AppState;
use crate::storage::{HistoryDetailEntity, HistoryRecordEntity, HistoryStorage};
use crate::utils::from_native_to_offset_time;

#[async_trait]
pub trait HistoryService {
    async fn create_text_to_speech_task(
        &self,
        request: CreateTextToSpeechTaskRequest,
    ) -> Result<HistoryRecordResponse>;
    async fn create_model_training_task(
        &self,
        request: CreateModelTrainingTaskRequest,
    ) -> Result<HistoryRecordResponse>;
    async fn create_voice_clone_task(
        &self,
        request: CreateVoiceCloneTaskRequest,
    ) -> Result<HistoryRecordResponse>;
    async fn create_voice_design_task(
        &self,
        request: CreateVoiceDesignTaskRequest,
    ) -> Result<VoiceDesignTaskResult>;
    async fn read_text_to_speech_audio(&self, history_id: i64) -> Result<TextToSpeechAudioAsset>;
    async fn read_voice_clone_audio(&self, history_id: i64) -> Result<VoiceCloneAudioAsset>;
    async fn read_voice_design_audio(&self, history_id: i64) -> Result<VoiceDesignAudioAsset>;
    async fn list_history_records(
        &self,
        request: PageRequest<HistoryListFilter>,
    ) -> Result<PageResponse<HistoryRecordSummaryResponse>>;
    async fn get_history_record(&self, history_id: i64) -> Result<HistoryRecordResponse>;
    async fn delete_history_record(
        &self,
        history_id: i64,
        task_type: crate::data_model::history::types::HistoryTaskType,
    ) -> Result<bool>;
    async fn cancel_history_task(&self, history_id: i64) -> Result<bool>;
}

#[async_trait]
impl HistoryService for AppState {
    async fn create_text_to_speech_task(
        &self,
        request: CreateTextToSpeechTaskRequest,
    ) -> Result<HistoryRecordResponse> {
        require_non_empty("任务标题", &request.title)?;
        require_non_empty("说话人名称", &request.speaker_name)?;
        require_non_empty("基础模型", &request.base_model)?;
        require_non_empty("模型版本", &request.model_version)?;
        require_non_empty("导出音频名", &request.export_audio_name)?;
        require_non_empty("文本", &request.text)?;

        let row = self
            .storage
            .create_text_to_speech_task(CreateTextToSpeechTaskDto {
                title: request.title,
                speaker_id: request.speaker_id,
                speaker_name: request.speaker_name,
                base_model: request.base_model,
                model_version: request.model_version,
                language: request.language,
                format: request.format,
                export_audio_name: request.export_audio_name,
                text: request.text,
                model_params: request.model_params,
                device: request.device,
            })
            .await?;
        map_history_record_entity(row)
    }

    async fn create_model_training_task(
        &self,
        request: CreateModelTrainingTaskRequest,
    ) -> Result<HistoryRecordResponse> {
        require_non_empty("任务标题", &request.title)?;
        require_non_empty("说话人名称", &request.speaker_name)?;
        require_non_empty("基础模型", &request.base_model)?;
        require_non_empty("模型版本", &request.model_version)?;
        if request.samples.is_empty() {
            bail!("训练样本不能为空");
        }

        let row = self
            .storage
            .create_model_training_task(CreateModelTrainingTaskDto {
                title: request.title,
                speaker_name: request.speaker_name,
                language: request.language,
                base_model: request.base_model,
                model_version: request.model_version,
                description: request.description,
                model_params: request.model_params,
                samples: request.samples,
                notes: request.notes,
                device: request.device,
            })
            .await?;
        map_history_record_entity(row)
    }

    async fn create_voice_clone_task(
        &self,
        request: CreateVoiceCloneTaskRequest,
    ) -> Result<HistoryRecordResponse> {
        require_non_empty("任务标题", &request.title)?;
        require_non_empty("说话人名称", &request.speaker_name)?;
        require_non_empty("基础模型", &request.base_model)?;
        require_non_empty("模型版本", &request.model_version)?;
        require_non_empty("导出音频名", &request.export_audio_name)?;
        require_non_empty("参考音频名称", &request.ref_audio_name)?;
        require_non_empty("参考音频路径", &request.ref_audio_path)?;
        require_non_empty("参考文本", &request.ref_text)?;
        require_non_empty("目标文本", &request.text)?;

        let row = self
            .storage
            .create_voice_clone_task(CreateVoiceCloneTaskDto {
                title: request.title,
                speaker_name: request.speaker_name,
                base_model: request.base_model,
                model_version: request.model_version,
                language: request.language,
                format: request.format,
                export_audio_name: request.export_audio_name,
                ref_audio_name: request.ref_audio_name,
                ref_audio_path: request.ref_audio_path,
                ref_text: request.ref_text,
                text: request.text,
                model_params: request.model_params,
                device: request.device,
            })
            .await?;
        map_history_record_entity(row)
    }

    async fn create_voice_design_task(
        &self,
        request: CreateVoiceDesignTaskRequest,
    ) -> Result<VoiceDesignTaskResult> {
        let CreateVoiceDesignTaskRequest {
            title,
            speaker_name,
            base_model,
            model_version,
            language,
            format,
            export_audio_name,
            prompt,
            text,
            model_params,
            device,
        } = request;
        let _ = (
            title,
            speaker_name,
            base_model,
            model_version,
            language,
            format,
            export_audio_name,
            prompt,
            text,
            model_params,
            device,
        );
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "声音设计任务暂未接入执行流水线，仅保留接口占位",
        )
        .into())
    }

    async fn read_text_to_speech_audio(&self, history_id: i64) -> Result<TextToSpeechAudioAsset> {
        let history = self.storage.get_history_record_entity(history_id).await?;
        match history.detail {
            HistoryDetailEntity::TextToSpeech(row) => Ok(TextToSpeechAudioAsset {
                task_id: history_id,
                file_name: row.output_file_path.clone().unwrap_or(row.export_audio_name),
                content_type: "audio/wav".to_string(),
                bytes: Vec::new(),
            }),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "任务类型不是文本转语音").into()),
        }
    }

    async fn read_voice_clone_audio(&self, history_id: i64) -> Result<VoiceCloneAudioAsset> {
        let history = self.storage.get_history_record_entity(history_id).await?;
        match history.detail {
            HistoryDetailEntity::VoiceClone(row) => Ok(VoiceCloneAudioAsset {
                task_id: history_id,
                file_name: row.output_file_path.clone().unwrap_or(row.export_audio_name),
                content_type: "audio/wav".to_string(),
                bytes: Vec::new(),
            }),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "任务类型不是声音克隆").into()),
        }
    }

    async fn read_voice_design_audio(&self, history_id: i64) -> Result<VoiceDesignAudioAsset> {
        let _ = history_id;
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "声音设计音频暂未接入执行流水线，仅保留接口占位",
        )
        .into())
    }

    async fn list_history_records(
        &self,
        request: PageRequest<HistoryListFilter>,
    ) -> Result<PageResponse<HistoryRecordSummaryResponse>> {
        let PageRequest {
            page,
            page_size,
            filter,
        } = request;

        let mut rows = self.storage.list_history_record_entities().await?;
        if let Some(keyword) = filter.keyword.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
            rows.retain(|record| {
                record.history.title.contains(keyword) || record.history.speaker_name.contains(keyword)
            });
        }
        if let Some(task_type) = filter.task_type {
            rows.retain(|record| match parse_history_task_type(&record.history.task_type) {
                Ok(value) => value == task_type,
                Err(_) => false,
            });
        }
        if let Some(status) = filter.status {
            rows.retain(|record| match parse_task_status(&record.history.status) {
                Ok(value) => value == status,
                Err(_) => false,
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
            .map(map_history_record_summary_entity)
            .collect::<Result<Vec<_>>>()?;

        Ok(PageResponse::new(total, page, page_size, items))
    }

    async fn get_history_record(&self, history_id: i64) -> Result<HistoryRecordResponse> {
        let row = self.storage.get_history_record_entity(history_id).await?;
        map_history_record_entity(row)
    }

    async fn delete_history_record(
        &self,
        history_id: i64,
        task_type: crate::data_model::history::types::HistoryTaskType,
    ) -> Result<bool> {
        let current = self.storage.get_history_record_entity(history_id).await?;
        let current_task_type = parse_history_task_type(&current.history.task_type)?;
        if current_task_type != task_type {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "任务类型与历史记录不匹配").into());
        }
        self.storage
            .mutate_history_record_state(history_id, HistoryRecordStateMutation::SoftDelete)
            .await
    }

    async fn cancel_history_task(&self, history_id: i64) -> Result<bool> {
        self.storage
            .mutate_history_record_state(history_id, HistoryRecordStateMutation::Cancel)
            .await
    }
}

fn map_history_record_entity(record: HistoryRecordEntity) -> Result<HistoryRecordResponse> {
    let row = record.history;
    Ok(HistoryRecordResponse {
        id: row.id,
        task_type: parse_history_task_type(&row.task_type)?,
        title: row.title,
        speaker: row.speaker_name,
        status: parse_task_status(&row.status)?,
        duration_seconds: row.duration_seconds,
        device: parse_hardware_type(&row.device)?,
        create_time: from_native_to_offset_time(row.create_time),
        modify_time: from_native_to_offset_time(row.modify_time),
        task_log: None,
        detail: map_history_detail_entity(record.detail)?,
    })
}

fn map_history_record_summary_entity(record: HistoryRecordEntity) -> Result<HistoryRecordSummaryResponse> {
    let row = record.history;
    Ok(HistoryRecordSummaryResponse {
        id: row.id,
        task_type: parse_history_task_type(&row.task_type)?,
        title: row.title,
        speaker: row.speaker_name,
        status: parse_task_status(&row.status)?,
        duration_seconds: row.duration_seconds,
        device: parse_hardware_type(&row.device)?,
        create_time: from_native_to_offset_time(row.create_time),
        modify_time: from_native_to_offset_time(row.modify_time),
    })
}

fn map_history_detail_entity(detail: HistoryDetailEntity) -> Result<Value> {
    match detail {
        HistoryDetailEntity::TextToSpeech(row) => {
            let export_audio_name = row.export_audio_name;
            Ok(serde_json::to_value(TextToSpeechTaskDetail {
                speaker_id: row.speaker_id,
                base_model: row.base_model,
                model_version: row.model_version,
                language: row
                    .language
                    .parse()
                    .map_err(|err: String| io::Error::new(io::ErrorKind::InvalidData, err))?,
                format: row
                    .format
                    .parse::<TextToSpeechFormat>()
                    .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?,
                export_audio_name: export_audio_name.clone(),
                text: row.text,
                model_params: serde_json::from_str(&row.model_params_json)?,
                char_count: row.char_count as usize,
                file_name: export_audio_name,
                output_file_path: row.output_file_path.unwrap_or_default(),
            })?)
        }
        HistoryDetailEntity::ModelTraining(row) => {
            Ok(serde_json::to_value(ModelTrainingTaskDetail {
                language: row
                    .language
                    .parse()
                    .map_err(|err: String| io::Error::new(io::ErrorKind::InvalidData, err))?,
                base_model: row.base_model,
                model_version: row.model_version,
                speaker_name: row.speaker_name,
                description: row.description,
                model_params: serde_json::from_str(&row.model_params_json)?,
                sample_count: row.sample_count,
                samples: serde_json::from_str::<Vec<ModelTrainingSampleInput>>(&row.samples_json)?,
                notes: serde_json::from_str(&row.notes_json)?,
            })?)
        }
        HistoryDetailEntity::VoiceClone(row) => {
            let export_audio_name = row.export_audio_name;
            Ok(serde_json::to_value(VoiceCloneTaskDetail {
                base_model: row.base_model,
                model_version: row.model_version,
                language: row
                    .language
                    .parse()
                    .map_err(|err: String| io::Error::new(io::ErrorKind::InvalidData, err))?,
                format: row
                    .format
                    .parse::<TextToSpeechFormat>()
                    .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?,
                export_audio_name: export_audio_name.clone(),
                ref_audio_name: row.ref_audio_name,
                ref_audio_path: row.ref_audio_path,
                ref_text: row.ref_text,
                text: row.text,
                model_params: serde_json::from_str(&row.model_params_json)?,
                char_count: row.char_count as usize,
                file_name: export_audio_name,
                output_file_path: row.output_file_path.unwrap_or_default(),
            })?)
        }
    }
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

fn parse_hardware_type(value: &str) -> Result<HardwareType> {
    value
        .parse::<HardwareType>()
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err).into())
}

fn require_non_empty(field_name: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        bail!("{}不能为空", field_name);
    }
    Ok(())
}
