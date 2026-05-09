use crate::api::entity::CommonResponse;

pub async fn login() -> CommonResponse<String> {
    CommonResponse::success(Some("login".to_string()))
}

pub async fn logout() -> CommonResponse<String> {
    CommonResponse::success(Some("logout".to_string()))
}