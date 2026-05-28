use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

use crate::error::Result;

/// 文件路径接口
///
/// 内部存储 vs 外部存储：
/// - **内部存储**：应用专属，其他应用不可访问，适合存放数据库、KV 存储等敏感数据
/// - **外部存储**：应用专属但位于共享存储区域，适合存放缓存、文件、日志等较大数据
///
/// 平台差异：
/// - Android: 内部 `/data/data/<pkg>/files`，外部 `/sdcard/Android/data/<pkg>/files`
/// - iOS/macOS/Linux/Windows: 无内外部区分，外部路径与内部路径相同
/// - OpenHarmony: 内部由 NDK API 获取，外部需要额外接口
pub trait PathProvider {
    // ===== 内部存储路径 =====

    /// 应用数据目录（内部存储，持久化）
    ///
    /// 适合存放：数据库、KV 存储、配置文件等敏感数据
    fn data_dir(&self) -> Result<PathBuf>;

    /// 缓存目录（内部存储，可清理）
    fn cache_dir(&self) -> Result<PathBuf>;

    /// 临时目录（系统可能清理）
    fn temp_dir(&self) -> Result<PathBuf>;

    /// 文档目录（用户可见）
    fn document_dir(&self) -> Result<PathBuf>;

    // ===== 外部存储路径 =====

    /// 应用数据目录（外部存储，持久化）
    ///
    /// 适合存放：日志、下载文件、用户数据等较大文件
    /// - Android: `/sdcard/Android/data/<pkg>/files`
    /// - 其他平台: 与 `data_dir()` 相同
    fn external_data_dir(&self) -> Result<PathBuf>;

    /// 缓存目录（外部存储，可清理）
    ///
    /// 适合存放：图片缓存、网络缓存等可重新生成的数据
    /// - Android: `/sdcard/Android/data/<pkg>/cache`
    /// - 其他平台: 与 `cache_dir()` 相同
    fn external_cache_dir(&self) -> Result<PathBuf>;

    // ===== 异步接口 =====

    /// 异步获取应用数据目录
    fn data_dir_async(&self) -> Pin<Box<dyn Future<Output = Result<PathBuf>> + Send + '_>> {
        Box::pin(std::future::ready(self.data_dir()))
    }

    /// 异步获取缓存目录
    fn cache_dir_async(&self) -> Pin<Box<dyn Future<Output = Result<PathBuf>> + Send + '_>> {
        Box::pin(std::future::ready(self.cache_dir()))
    }

    /// 异步获取临时目录
    fn temp_dir_async(&self) -> Pin<Box<dyn Future<Output = Result<PathBuf>> + Send + '_>> {
        Box::pin(std::future::ready(self.temp_dir()))
    }

    /// 异步获取文档目录
    fn document_dir_async(&self) -> Pin<Box<dyn Future<Output = Result<PathBuf>> + Send + '_>> {
        Box::pin(std::future::ready(self.document_dir()))
    }

    /// 异步获取外部存储应用数据目录
    fn external_data_dir_async(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<PathBuf>> + Send + '_>> {
        Box::pin(std::future::ready(self.external_data_dir()))
    }

    /// 异步获取外部存储缓存目录
    fn external_cache_dir_async(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<PathBuf>> + Send + '_>> {
        Box::pin(std::future::ready(self.external_cache_dir()))
    }
}
