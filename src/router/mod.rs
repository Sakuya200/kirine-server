use axum::Router;
use axum::routing::post;
use crate::api::auth::{auth_middleware, AuthApi};
use crate::api::speaker::SpeakerApi;
use crate::config::EnvConfig;
use crate::service::AppState;
use crate::storage::LocalStorage;

pub fn init_router(env_config: EnvConfig, local_storage: LocalStorage) -> Router {
    let app_state = AppState::new(env_config, local_storage);

    Router::new()
        .nest("/speaker", speaker_api(app_state.clone()))
        .nest("/auth", auth_api(app_state.clone()))
        .with_state(app_state)
}


pub fn auth_api(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/logout", post(AuthApi::logout))
        .route_layer(axum::middleware::from_fn_with_state(app_state, auth_middleware))
        .route("/login", post(AuthApi::login))
}

pub fn speaker_api(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/create", post(SpeakerApi::create_speaker_info))
        .route("/import_model", post(SpeakerApi::import_model_as_speaker))
        .route("/list", post(SpeakerApi::list_speaker_infos))
        .route("/update", post(SpeakerApi::update_speaker_info))
        .route("/delete/{speaker_id}", post(SpeakerApi::delete_speaker_info))
        .route_layer(axum::middleware::from_fn_with_state(app_state.clone(), auth_middleware))
}