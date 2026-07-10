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
            io::ErrorKind::Unsupported => 501,
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

/// 分页请求结构体
/// page: 页码
/// page_size: 每页条数
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageRequest<T> {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
    #[serde(flatten)]
    pub filter: T,
}

/// 分页响应结构体
/// total: 总条数
/// page: 当前页码
/// page_size: 每页条数
/// total_pages: 总页数
/// items: 当前页数据
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse<T> {
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
    pub items: Vec<T>,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    10
}

impl<T> PageResponse<T> {
    pub fn new(total: u64, page: u32, page_size: u32, items: Vec<T>) -> Self {
        let total_pages = if total == 0 {
            0
        } else {
            ((total + page_size as u64 - 1) / page_size as u64) as u32
        };

        Self {
            total,
            page,
            page_size,
            total_pages,
            items,
        }
    }
}
