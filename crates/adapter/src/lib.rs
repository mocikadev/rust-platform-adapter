pub mod error;
pub mod types;
pub mod traits;
pub mod platform;

// 便捷 API 模块
mod api;

// re-export 公共接口
pub use error::PlatformError;
pub use types::*;
pub use traits::*;

// re-export 便捷 API
pub use api::*;

/// 预导入模块
pub mod prelude {
    pub use crate::error::{PlatformError, Result};
    pub use crate::types::*;
    pub use crate::traits::*;
    pub use crate::api::*;
}
