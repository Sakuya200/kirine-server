use crate::api::entity::CommonResponse;
use crate::data_model::speaker::req::{
    CreateSpeakerRequest, ImportModelAsSpeakerRequest, UpdateSpeakerRequest,
};
use crate::data_model::speaker::resp::SpeakerInfoResponse;
use crate::service::{AppState, SpeakerService};
use axum::extract::{Path, State};
use axum::Json;

pub struct SpeakerApi {}

impl SpeakerApi {
    pub async fn create_speaker_info(
        State(app_state): State<AppState>,
        Json(request): Json<CreateSpeakerRequest>,
    ) -> CommonResponse<SpeakerInfoResponse> {
        CommonResponse::from_result(app_state.create_speaker_info(request).await)
    }

    pub async fn import_model_as_speaker(
        State(app_state): State<AppState>,
        Json(request): Json<ImportModelAsSpeakerRequest>,
    ) -> CommonResponse<SpeakerInfoResponse> {
        CommonResponse::from_result(app_state.import_model_as_speaker(request).await)
    }

    pub async fn list_speaker_infos(
        State(app_state): State<AppState>,
    ) -> CommonResponse<Vec<SpeakerInfoResponse>> {
        CommonResponse::from_result(app_state.list_speaker_infos().await)
    }

    pub async fn update_speaker_info(
        State(app_state): State<AppState>,
        Json(request): Json<UpdateSpeakerRequest>,
    ) -> CommonResponse<SpeakerInfoResponse> {
        CommonResponse::from_result(app_state.update_speaker_info(request).await)
    }

    pub async fn delete_speaker_info(
        State(app_state): State<AppState>,
        Path(speaker_id): Path<i64>,
    ) -> CommonResponse<bool> {
        CommonResponse::from_result(app_state.delete_speaker_info(speaker_id).await)
    }
}
