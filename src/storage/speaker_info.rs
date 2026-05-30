use crate::models::speaker::{
    CreateSpeakerRequest, ImportModelAsSpeakerRequest, SpeakerInfoResponse, UpdateSpeakerRequest,
};
use crate::storage::LocalStorage;
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait SpeakerInfoStorage {
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
    ) -> anyhow::Result<SpeakerInfoResponse>;

    async fn delete_speaker_info(&self, speaker_id: i64) -> Result<bool>;
}

#[async_trait]
impl SpeakerInfoStorage for LocalStorage {
    async fn create_speaker_info(&self, request: CreateSpeakerRequest) -> Result<SpeakerInfoResponse> {
        todo!()
    }

    async fn import_model_as_speaker(&self, request: ImportModelAsSpeakerRequest) -> Result<SpeakerInfoResponse> {
        todo!()
    }

    async fn list_speaker_infos(&self) -> Result<Vec<SpeakerInfoResponse>> {
        todo!()
    }

    async fn update_speaker_info(&self, request: UpdateSpeakerRequest) -> Result<SpeakerInfoResponse> {
        todo!()
    }

    async fn delete_speaker_info(&self, speaker_id: i64) -> Result<bool> {
        todo!()
    }
}
