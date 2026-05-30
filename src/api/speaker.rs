use crate::models::speaker::{CreateSpeakerRequest, ImportModelAsSpeakerRequest, SpeakerInfoResponse, UpdateSpeakerRequest};
use crate::api::entity::CommonResponse;

pub struct SpeakerApi {}

impl SpeakerApi {
    pub async fn create_speaker_info(&self, request: CreateSpeakerRequest) -> CommonResponse<SpeakerInfoResponse> {
        todo!()
    }

    pub async fn import_model_as_speaker(
        &self,
        request: ImportModelAsSpeakerRequest,
    ) -> CommonResponse<SpeakerInfoResponse> {
        todo!()
    }

    pub async fn list_speaker_infos(&self) -> CommonResponse<Vec<SpeakerInfoResponse>> {
        todo!()
    }

    pub async fn update_speaker_info(&self, request: UpdateSpeakerRequest) -> CommonResponse<SpeakerInfoResponse> {
        todo!()
    }

    pub async fn delete_speaker_info(&self, speaker_id: i64) -> CommonResponse<bool> {
        todo!()
    }
}