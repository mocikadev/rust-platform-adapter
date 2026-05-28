use thiserror::Error;

/// 平台适配错误类型
#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("当前平台不支持此功能")]
    NotSupported,

    #[error("权限不足: {0}")]
    PermissionDenied(String),

    #[error("系统错误: {0}")]
    SystemError(i32),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("FFI 错误: {0}")]
    FfiError(String),
}

pub type Result<T> = std::result::Result<T, PlatformError>;
