use std::path::PathBuf;

use crate::error::Result;

/// 文件路径接口
pub trait PathProvider {
    /// 应用数据目录（持久化存储）
    fn data_dir(&self) -> Result<PathBuf>;

    /// 缓存目录（可清理）
    fn cache_dir(&self) -> Result<PathBuf>;

    /// 临时目录（系统可能清理）
    fn temp_dir(&self) -> Result<PathBuf>;

    /// 文档目录（用户可见）
    fn document_dir(&self) -> Result<PathBuf>;
}
