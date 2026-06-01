use crate::api::auth::{AuthApi, auth_middleware};
use crate::api::history::HistoryApi;
use crate::api::model_info::ModelInfoApi;
use crate::api::speaker::SpeakerApi;
use crate::config::EnvConfig;
use crate::service::AppState;
use crate::storage::LocalStorage;
use axum::Router;
use axum::routing::post;

pub fn init_router(env_config: EnvConfig, local_storage: LocalStorage) -> Router {
    let app_state = AppState::new(env_config, local_storage);

    Router::new()
        .nest("/model", model_api(app_state.clone()))
        .nest("/history", history_api(app_state.clone()))
        .nest("/speaker", speaker_api(app_state.clone()))
        .nest("/auth", auth_api(app_state.clone()))
        .with_state(app_state)
}

pub fn auth_api(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/logout", post(AuthApi::logout))
        .route_layer(axum::middleware::from_fn_with_state(
            app_state,
            auth_middleware,
        ))
        .route("/login", post(AuthApi::login))
}

pub fn speaker_api(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/create", post(SpeakerApi::create_speaker_info))
        .route("/import_model", post(SpeakerApi::import_model_as_speaker))
        .route("/list", post(SpeakerApi::list_speaker_infos))
        .route("/update", post(SpeakerApi::update_speaker_info))
        .route(
            "/delete/{speaker_id}",
            post(SpeakerApi::delete_speaker_info),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
}

pub fn model_api(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/list", post(ModelInfoApi::list_model_infos))
        .route("/detail/{model_id}", post(ModelInfoApi::get_model_info))
        .route("/device_type", post(ModelInfoApi::get_device_type))
        .route(
            "/install/{model_id}/{device}",
            post(ModelInfoApi::install_model),
        )
        .route("/uninstall/{model_id}", post(ModelInfoApi::uninstall_model))
        .route_layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
}

pub fn history_api(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/list", post(HistoryApi::list_history_records))
        .route("/detail/{history_id}", post(HistoryApi::get_history_record))
        .route(
            "/delete/{history_id}",
            post(HistoryApi::delete_history_record),
        )
        .route(
            "/cancel/{history_id}",
            post(HistoryApi::cancel_history_task),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
}
