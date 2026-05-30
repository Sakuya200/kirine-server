use crate::models::speaker::{
    CreateSpeakerRequest, ImportModelAsSpeakerRequest, SpeakerInfoResponse, UpdateSpeakerRequest,
};
use crate::storage::LocalStorage;
use async_trait::async_trait;

#[async_trait]
pub trait SpeakerInfoStorage {
    async fn create_speaker_info(
        &self,
        request: CreateSpeakerRequest,
    ) -> anyhow::Result<SpeakerInfoResponse>;

    async fn import_model_as_speaker(
        &self,
        request: ImportModelAsSpeakerRequest,
    ) -> anyhow::Result<SpeakerInfoResponse>;

    async fn list_speaker_infos(&self) -> anyhow::Result<Vec<SpeakerInfoResponse>>;

    async fn update_speaker_info(
        &self,
        request: UpdateSpeakerRequest,
    ) -> anyhow::Result<SpeakerInfoResponse>;

    async fn delete_speaker_info(&self, speaker_id: i64) -> anyhow::Result<bool>;
}

#[async_trait]
impl SpeakerInfoStorage for LocalStorage {
    async fn create_speaker_info(&self, request: CreateSpeakerRequest) -> anyhow::Result<SpeakerInfoResponse> {
        todo!()
    }

    async fn import_model_as_speaker(&self, request: ImportModelAsSpeakerRequest) -> anyhow::Result<SpeakerInfoResponse> {
        todo!()
    }

    async fn list_speaker_infos(&self) -> anyhow::Result<Vec<SpeakerInfoResponse>> {
        todo!()
    }

    async fn update_speaker_info(&self, request: UpdateSpeakerRequest) -> anyhow::Result<SpeakerInfoResponse> {
        todo!()
    }

    async fn delete_speaker_info(&self, speaker_id: i64) -> anyhow::Result<bool> {
        todo!()
    }
}
