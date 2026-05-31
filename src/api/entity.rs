use anyhow::Result;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::io;
use tracing::log;

use crate::api::auth::AuthError;

/// 通用响应结构体
/// status_code: 状态码
/// data: 响应数据
/// message: 响应消息
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonResponse<T: Serialize> {
    pub code: i32,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T: Serialize> CommonResponse<T> {
    pub fn new(status_code: i32, data: Option<T>, message: Option<String>) -> Self {
        CommonResponse {
            code: status_code,
            data,
            message,
        }
    }

    pub fn success(data: Option<T>) -> Self {
        CommonResponse {
            code: 200,
            data,
            message: None,
        }
    }

    pub fn error(status_code: i32, message: String) -> Self {
        CommonResponse {
            code: status_code,
            data: None,
            message: Some(message),
        }
    }

    pub fn from_result(res: Result<T>) -> Self {
        match res {
            Ok(data) => CommonResponse::success(Some(data)),
            Err(e) => {
                let code = map_error_code(&e);
                CommonResponse::error(code, e.to_string())
            }
        }
    }
}

fn map_error_code(error: &anyhow::Error) -> i32 {
    if let Some(io_error) = error.downcast_ref::<io::Error>() {
        return match io_error.kind() {
            io::ErrorKind::InvalidInput => 400,
            io::ErrorKind::PermissionDenied => 403,
            io::ErrorKind::NotFound => 404,
            io::ErrorKind::AlreadyExists => 409,
            _ => 500,
        };
    }

    if let Some(auth_error) = error.downcast_ref::<AuthError>() {
        return match auth_error {
            AuthError::InvalidToken => 401,
            AuthError::MissingToken => 401,
            AuthError::Forbidden => 403,
        };
    }

    500
}

impl<T: Serialize> IntoResponse for CommonResponse<T> {
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self).unwrap_or_else(|e| {
            log::error!("Failed to serialize CommonResponse: {e}");
            serde_json::to_string(&CommonResponse::<T>::error(
                500,
                "Failed to serialize response".to_string(),
            ))
            .unwrap()
        });
        Response::builder()
            .header("Content-Type", "application/json")
            .body(body.into())
            .unwrap()
    }
}
