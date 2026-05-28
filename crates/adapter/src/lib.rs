pub mod error;
pub mod platform;
pub mod traits;
pub mod types;

// 便捷 API 模块
mod api;

// re-export 公共接口
pub use error::PlatformError;
pub use traits::*;
pub use types::*;

// re-export 便捷 API
pub use api::*;

/// 预导入模块
pub mod prelude {
    pub use crate::api::*;
    pub use crate::error::{PlatformError, Result};
    pub use crate::traits::*;
    pub use crate::types::*;
}
