use crate::api::auth::{AuthApi, auth_middleware};
use crate::api::history::HistoryApi;
use crate::api::model_info::ModelInfoApi;
use crate::api::speaker::SpeakerApi;
use crate::config::EnvConfig;
use crate::service::AppState;
use crate::storage::LocalStorage;
use axum::Router;
use axum::routing::{get, post, put};

pub fn init_router(env_config: EnvConfig, local_storage: LocalStorage) -> Router {
    let app_state = AppState::new(env_config, local_storage);

    Router::new()
        .nest("/models", model_api(app_state.clone()))
        .nest("/history", history_api(app_state.clone()))
        .nest("/speakers", speaker_api(app_state.clone()))
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
        .route("", post(SpeakerApi::create_speaker_info).get(SpeakerApi::list_speaker_infos))
        .route("/import", post(SpeakerApi::import_model_as_speaker))
        .route(
            "/{id}",
            put(SpeakerApi::update_speaker_info).delete(SpeakerApi::delete_speaker_info),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
}

pub fn model_api(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("", get(ModelInfoApi::list_model_infos))
        .route("/device-type", get(ModelInfoApi::get_device_type))
        .route(
            "/{id}",
            get(ModelInfoApi::get_model_info).delete(ModelInfoApi::uninstall_model),
        )
        .route("/{id}/install", post(ModelInfoApi::install_model))
        .route_layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
}

pub fn history_api(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("", get(HistoryApi::list_history_records))
        .route("/voice-design", post(HistoryApi::create_voice_design_task))
        .route(
            "/{id}",
            get(HistoryApi::get_history_record).delete(HistoryApi::delete_history_record),
        )
        .route("/{id}/status", put(HistoryApi::update_task_status))
        .route("/{id}/cancel", post(HistoryApi::cancel_history_task))
        .route("/{id}/audio/text-to-speech", get(HistoryApi::get_history_audio))
        .route("/{id}/audio/voice-clone", get(HistoryApi::get_voice_clone_audio))
        .route("/{id}/audio/voice-design", get(HistoryApi::get_voice_design_audio))
        .route_layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
}
