use crate::error::ApiError;
use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct R<T: Serialize> {
    pub code: u8,
    pub msg: String,
    pub data: Option<T>,
}
impl<T: Serialize> R<T> {
    ///不带msg和数据的成功
    pub fn ok_empty() -> Self {
        Self {
            code: 0,
            msg: "".to_string(),
            data: None,
        }
    }
    ///带数据的成功
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            msg: "操作成功".to_string(),
            data: Some(data),
        }
    }
    ///带数据带自定义内容的成功
    pub fn ok_msg(data: T, msg: impl Into<String>) -> Self {
        Self {
            code: 0,
            msg: msg.into(),
            data: Some(data),
        }
    }
    ///失败,需要返回错误提示
    pub fn err(msg: impl Into<String>) -> R<()> {
        R {
            code: 1,
            msg: msg.into(),
            data: None,
        }
    }
}
/// 让 R 可以直接从 axum 处理器返回
impl<T: Serialize> IntoResponse for R<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
/// 让 AppError 可以直接从 axum 处理器返回
/// 自动转换成对应 HTTP 状态码 + 统一响应格式
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = R::<()>::err(self.to_string());
        (status, Json(body)).into_response()
    }
}
