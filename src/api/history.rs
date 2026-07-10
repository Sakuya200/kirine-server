use axum::extract::{Path, State};
use axum::extract::Query;
use axum::Json;

use crate::api::entity::CommonResponse;
use crate::api::entity::{PageRequest, PageResponse};
use crate::data_model::history::req::{
    CreateVoiceDesignTaskRequest, DeleteHistoryQuery, HistoryListFilter, UpdateTaskStatusRequest,
};
use crate::data_model::history::resp::{
    HistoryRecordResponse, HistoryRecordSummaryResponse, TextToSpeechAudioAsset,
    VoiceCloneAudioAsset, VoiceDesignAudioAsset, VoiceDesignTaskResult,
};
use crate::service::{AppState, HistoryService};

pub struct HistoryApi;

impl HistoryApi {
    pub async fn list_history_records(
        State(app_state): State<AppState>,
        Query(request): Query<PageRequest<HistoryListFilter>>,
    ) -> CommonResponse<PageResponse<HistoryRecordSummaryResponse>> {
        CommonResponse::from_result(app_state.list_history_records(request).await)
    }

    pub async fn get_history_audio(
        State(app_state): State<AppState>,
        Path(history_id): Path<i64>,
    ) -> CommonResponse<TextToSpeechAudioAsset> {
        CommonResponse::from_result(app_state.read_text_to_speech_audio(history_id).await)
    }

    pub async fn get_voice_clone_audio(
        State(app_state): State<AppState>,
        Path(history_id): Path<i64>,
    ) -> CommonResponse<VoiceCloneAudioAsset> {
        CommonResponse::from_result(app_state.read_voice_clone_audio(history_id).await)
    }

    pub async fn get_voice_design_audio(
        State(app_state): State<AppState>,
        Path(history_id): Path<i64>,
    ) -> CommonResponse<VoiceDesignAudioAsset> {
        CommonResponse::from_result(app_state.read_voice_design_audio(history_id).await)
    }

    pub async fn get_history_record(
        State(app_state): State<AppState>,
        Path(history_id): Path<i64>,
    ) -> CommonResponse<HistoryRecordResponse> {
        CommonResponse::from_result(app_state.get_history_record(history_id).await)
    }

    pub async fn delete_history_record(
        State(app_state): State<AppState>,
        Path(history_id): Path<i64>,
        Query(query): Query<DeleteHistoryQuery>,
    ) -> CommonResponse<bool> {
        CommonResponse::from_result(app_state.delete_history_record(history_id, query.task_type).await)
    }

    pub async fn update_task_status(
        State(app_state): State<AppState>,
        Path(history_id): Path<i64>,
        Json(request): Json<UpdateTaskStatusRequest>,
    ) -> CommonResponse<HistoryRecordResponse> {
        let UpdateTaskStatusRequest {
            task_id,
            status,
            duration_seconds,
        } = request;
        let _ = (app_state, history_id, task_id, status, duration_seconds);
        CommonResponse::from_result(Err::<HistoryRecordResponse, _>(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            format!("任务状态回写暂未接入执行流水线，仅保留接口占位: task_id={task_id}"),
        )
        .into()))
    }

    pub async fn create_voice_design_task(
        State(app_state): State<AppState>,
        Json(request): Json<CreateVoiceDesignTaskRequest>,
    ) -> CommonResponse<VoiceDesignTaskResult> {
        CommonResponse::from_result(app_state.create_voice_design_task(request).await)
    }

    pub async fn cancel_history_task(
        State(app_state): State<AppState>,
        Path(history_id): Path<i64>,
    ) -> CommonResponse<bool> {
        CommonResponse::from_result(app_state.cancel_history_task(history_id).await)
    }
}
