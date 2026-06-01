use axum::extract::{Path, State};
use axum::Json;

use crate::api::entity::CommonResponse;
use crate::data_model::model_info::req::GetDeviceTypeRequest;
use crate::data_model::model_info::resp::{ModelInfoResponse, ModelMutationResult};
use crate::pipeline::HardwareType;
use crate::service::{AppState, ModelInfoService};

pub struct ModelInfoApi;

impl ModelInfoApi {
    pub async fn list_model_infos(
        State(app_state): State<AppState>,
    ) -> CommonResponse<Vec<ModelInfoResponse>> {
        CommonResponse::from_result(app_state.list_model_infos().await)
    }

    pub async fn get_device_type(
        State(app_state): State<AppState>,
        Json(request): Json<GetDeviceTypeRequest>,
    ) -> CommonResponse<HardwareType> {
        CommonResponse::from_result(
            app_state
                .get_device_type(&request.base_model, &request.model_version)
                .await,
        )
    }

    pub async fn install_model(
        State(app_state): State<AppState>,
        Path((model_id, device)): Path<(i64, HardwareType)>,
    ) -> CommonResponse<ModelMutationResult> {
        CommonResponse::from_result(app_state.install_model(model_id, device).await)
    }

    pub async fn uninstall_model(
        State(app_state): State<AppState>,
        Path(model_id): Path<i64>,
    ) -> CommonResponse<ModelMutationResult> {
        CommonResponse::from_result(app_state.uninstall_model(model_id).await)
    }

    pub async fn get_model_info(
        State(app_state): State<AppState>,
        Path(model_id): Path<i64>,
    ) -> CommonResponse<ModelInfoResponse> {
        CommonResponse::from_result(app_state.get_model_info(model_id).await)
    }
}
