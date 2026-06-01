use axum::Json;
use axum::extract::{Path, State};

use crate::api::entity::CommonResponse;
use crate::data_model::history::req::{
    CreateModelTrainingTaskRequest, CreateTextToSpeechTaskRequest, CreateVoiceCloneTaskRequest,
};
use crate::data_model::history::resp::HistoryRecordResponse;
use crate::service::{AppState, HistoryService};

pub struct HistoryApi;

impl HistoryApi {
    pub async fn list_history_records(
        State(app_state): State<AppState>,
    ) -> CommonResponse<Vec<HistoryRecordResponse>> {
        CommonResponse::from_result(app_state.list_history_records().await)
    }

    pub async fn get_history_audio(
        State(app_state): State<AppState>,
        Path(history_id): Path<i64>,
    ) -> CommonResponse<String> {
        CommonResponse::from_result(app_state.get_history_audio(history_id).await)
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
    ) -> CommonResponse<bool> {
        CommonResponse::from_result(app_state.delete_history_record(history_id).await)
    }

    pub async fn cancel_history_task(
        State(app_state): State<AppState>,
        Path(history_id): Path<i64>,
    ) -> CommonResponse<bool> {
        CommonResponse::from_result(app_state.cancel_history_task(history_id).await)
    }
}
