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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_supported_error() {
        let err = PlatformError::NotSupported;
        assert_eq!(err.to_string(), "当前平台不支持此功能");
    }

    #[test]
    fn test_permission_denied_error() {
        let err = PlatformError::PermissionDenied("storage".to_string());
        assert!(err.to_string().contains("storage"));
    }

    #[test]
    fn test_system_error() {
        let err = PlatformError::SystemError(42);
        assert!(err.to_string().contains("42"));
    }

    #[test]
    fn test_ffi_error() {
        let err = PlatformError::FfiError("something failed".to_string());
        assert!(err.to_string().contains("something failed"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let platform_err: PlatformError = io_err.into();
        assert!(matches!(platform_err, PlatformError::IoError(_)));
    }

    #[test]
    fn test_result_ok() {
        let val: Result<i32> = Ok(42);
        match val {
            Ok(v) => assert_eq!(v, 42),
            Err(_) => panic!("expected Ok"),
        }
    }

    #[test]
    fn test_result_err() {
        let result: Result<i32> = Err(PlatformError::NotSupported);
        assert!(result.is_err());
    }
}
