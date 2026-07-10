use axum::extract::{Path, Query, State};

use crate::api::entity::CommonResponse;
use crate::api::entity::{PageRequest, PageResponse};
use crate::data_model::model_info::req::{GetDeviceTypeRequest, InstallModelQuery, ModelListFilter};
use crate::data_model::model_info::resp::{ModelInfoResponse, ModelMutationResult};
use crate::pipeline::HardwareType;
use crate::service::{AppState, ModelInfoService};

pub struct ModelInfoApi;

impl ModelInfoApi {
    pub async fn list_model_infos(
        State(app_state): State<AppState>,
        Query(request): Query<PageRequest<ModelListFilter>>,
    ) -> CommonResponse<PageResponse<ModelInfoResponse>> {
        CommonResponse::from_result(app_state.list_model_infos(request).await)
    }

    pub async fn get_device_type(
        State(app_state): State<AppState>,
        Query(request): Query<GetDeviceTypeRequest>,
    ) -> CommonResponse<HardwareType> {
        CommonResponse::from_result(
            app_state
                .get_device_type(&request.base_model, &request.model_version)
                .await,
        )
    }

    pub async fn install_model(
        State(app_state): State<AppState>,
        Path(model_id): Path<i64>,
        Query(query): Query<InstallModelQuery>,
    ) -> CommonResponse<ModelMutationResult> {
        CommonResponse::from_result(app_state.install_model(model_id, query.device).await)
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
