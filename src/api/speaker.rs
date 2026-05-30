use crate::models::speaker::{CreateSpeakerRequest, ImportModelAsSpeakerRequest, SpeakerInfoResponse, UpdateSpeakerRequest};
use anyhow::Result;
pub struct SpeakerApi {}

impl SpeakerApi {
    pub async fn create_speaker_info(&self, request: CreateSpeakerRequest) -> Result<SpeakerInfoResponse> {
        todo!()
    }

    pub async fn import_model_as_speaker(
        &self,
        request: ImportModelAsSpeakerRequest,
    ) -> Result<SpeakerInfoResponse> {
        todo!()
    }

    pub async fn list_speaker_infos(&self) -> Result<Vec<SpeakerInfoResponse>> {
        todo!()
    }

    pub async fn update_speaker_info(&self, request: UpdateSpeakerRequest) -> Result<SpeakerInfoResponse> {
        todo!()
    }

    pub async fn delete_speaker_info(&self, speaker_id: i64) -> Result<bool> {
        todo!()
    }
}