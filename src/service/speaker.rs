use crate::models::speaker::{CreateSpeakerRequest, ImportModelAsSpeakerRequest, SpeakerInfoResponse, UpdateSpeakerRequest};
use crate::service::AppState;
use anyhow::Result;
use async_trait::async_trait;
use crate::storage::SpeakerInfoStorage;

#[async_trait]
pub trait SpeakerService {
    async fn create_speaker_info(
        &self,
        request: CreateSpeakerRequest,
    ) -> Result<SpeakerInfoResponse>;

    async fn import_model_as_speaker(
        &self,
        request: ImportModelAsSpeakerRequest,
    ) -> Result<SpeakerInfoResponse>;

    async fn list_speaker_infos(&self) -> Result<Vec<SpeakerInfoResponse>>;

    async fn update_speaker_info(
        &self,
        request: UpdateSpeakerRequest,
    ) -> Result<SpeakerInfoResponse>;

    async fn delete_speaker_info(&self, speaker_id: i64) -> Result<bool>;
}

#[async_trait]
impl SpeakerService for AppState {
    async fn create_speaker_info(
        &self,
        request: CreateSpeakerRequest,
    ) -> Result<SpeakerInfoResponse> {
        self.storage.create_speaker_info(request).await
    }

    async fn import_model_as_speaker(
        &self,
        request: ImportModelAsSpeakerRequest,
    ) -> Result<SpeakerInfoResponse> {
        self.storage.import_model_as_speaker(request).await
    }

    async fn list_speaker_infos(&self) -> Result<Vec<SpeakerInfoResponse>> {
        self.storage.list_speaker_infos().await
    }

    async fn update_speaker_info(
        &self,
        request: UpdateSpeakerRequest,
    ) -> Result<SpeakerInfoResponse> {
        self.storage.update_speaker_info(request).await
    }

    async fn delete_speaker_info(&self, speaker_id: i64) -> Result<bool> {
        self.storage.delete_speaker_info(speaker_id).await
    }
}
