use axum::http::StatusCode;
use thiserror::Error;
/// 应用统一错误类型
#[derive(Debug, Error)]
pub enum ApiError {
    // ── 业务错误 ──────────────────────────────────────
    /// 400 请求参数错误
    #[error("{0}")]
    BadRequest(String),

    /// 401 未登录或 token 无效
    #[error("{0}")]
    Unauthorized(String),

    /// 403 无权限
    #[error("{0}")]
    Forbidden(String),

    /// 404 资源不存在
    #[error("{0}")]
    NotFound(String),

    /// 通用业务异常
    /// 用于业务逻辑中直接抛出的错误
    #[error("{0}")]
    BizError(String),

    // ── 系统错误 ──────────────────────────────────────
    /// 数据库错误
    #[error("数据库错误: {0}")]
    DbError(#[from] sea_orm::DbErr),

    /// 配置错误
    #[error("配置错误: {0}")]
    ConfigError(#[from] config::ConfigError),

    /// 内部服务错误（兜底）
    #[error("内部服务错误: {0}")]
    InternalError(String),
}

impl ApiError {
    /// 获取对应的 HTTP 状态码
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST, // 400
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED, // 401
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,    // 403
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,     // 404
            ApiError::BizError(_) => StatusCode::BAD_REQUEST,   // 400
            ApiError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR, // 500
            ApiError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR, // 500
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR, // 500
        }
    }

    /// 是否是系统级错误,系统错误需要打印完整堆栈
    pub fn is_system_error(&self) -> bool {
        matches!(
            self,
            ApiError::DbError(_) | ApiError::ConfigError(_) | ApiError::InternalError(_)
        )
    }
}
