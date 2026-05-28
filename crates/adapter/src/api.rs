use std::path::PathBuf;

use crate::error::Result;
use crate::traits::{DeviceInfo, PathProvider, ScreenProvider};
use crate::types::{
    CpuArch, DeviceForm, Orientation, OsType, PlatformInfo, ScreenInfo, CURRENT_OS,
};

// ========== 条件编译宏 ==========

/// 编译时平台条件编译宏
///
/// 提供比 `#[cfg(target_os = "...")]` 更简洁的语法糖，
/// 编译期完成分发，零运行时开销。
///
/// # 支持的平台标识
///
/// 单平台：`android` / `ios` / `ohos` / `windows` / `linux` / `macos`
/// 分组：`mobile`（android + ios + ohos）/ `desktop`（windows + linux + macos）
///
/// # 示例
///
/// ```ignore
/// platform!(android => {
///     println!("Running on Android!");
/// });
///
/// platform!(mobile => {
///     // 移动平台特有逻辑
/// });
/// ```
#[macro_export]
macro_rules! platform {
    (android => $block:block) => {
        #[cfg(target_os = "android")]
        $block
    };
    (ios => $block:block) => {
        #[cfg(target_os = "ios")]
        $block
    };
    (ohos => $block:block) => {
        #[cfg(target_os = "ohos")]
        $block
    };
    (windows => $block:block) => {
        #[cfg(target_os = "windows")]
        $block
    };
    (linux => $block:block) => {
        #[cfg(target_os = "linux")]
        $block
    };
    (macos => $block:block) => {
        #[cfg(target_os = "macos")]
        $block
    };
    (mobile => $block:block) => {
        #[cfg(any(target_os = "android", target_os = "ios", target_os = "ohos"))]
        $block
    };
    (desktop => $block:block) => {
        #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
        $block
    };
}

// ========== 运行时能力检测 ==========

/// 检测当前环境是否有可用的图形显示（有头环境）
///
/// - Linux: 检查 `DISPLAY` 环境变量
/// - Windows/macOS: 始终返回 `true`（有虚拟桌面/显示器）
/// - 移动平台: 始终返回 `true`
pub fn has_display() -> bool {
    #[cfg(target_os = "linux")]
    {
        std::env::var("DISPLAY").is_ok()
    }
    #[cfg(not(target_os = "linux"))]
    {
        true
    }
}

// ========== 系统类型判断（编译时常量） ==========

/// 获取当前操作系统类型（编译时确定）
pub const fn current_os() -> OsType {
    CURRENT_OS
}

/// 是否为 Android
pub const fn is_android() -> bool {
    CURRENT_OS.is_android()
}

/// 是否为 iOS
pub const fn is_ios() -> bool {
    CURRENT_OS.is_ios()
}

/// 是否为 OpenHarmony / HarmonyOS
pub const fn is_ohos() -> bool {
    CURRENT_OS.is_ohos()
}

/// 是否为 Windows
pub const fn is_windows() -> bool {
    CURRENT_OS.is_windows()
}

/// 是否为 Linux
pub const fn is_linux() -> bool {
    CURRENT_OS.is_linux()
}

/// 是否为 macOS
pub const fn is_macos() -> bool {
    CURRENT_OS.is_macos()
}

// ========== 平台信息 ==========

/// 获取平台信息
pub fn platform_info() -> Result<PlatformInfo> {
    crate::platform::device_provider().platform_info()
}

/// 获取操作系统版本
pub fn os_version() -> Result<String> {
    crate::platform::device_provider().os_version()
}

/// 获取设备型号
pub fn device_model() -> Result<String> {
    crate::platform::device_provider().device_model()
}

/// 获取 CPU 架构
pub fn cpu_arch() -> CpuArch {
    crate::platform::device_provider().cpu_arch()
}

/// 获取设备形态
pub fn device_form() -> DeviceForm {
    crate::platform::device_provider().device_form()
}

/// 获取当前设备形态（与 device_form 相同，语义更明确）
pub fn current_device_form() -> DeviceForm {
    device_form()
}

/// 是否为手机
pub fn is_phone() -> bool {
    device_form().is_phone()
}

/// 是否为平板
pub fn is_tablet() -> bool {
    device_form().is_tablet()
}

/// 是否为桌面设备
pub fn is_desktop_device() -> bool {
    device_form().is_desktop()
}

// ========== 路径相关 ==========

/// 获取应用数据目录
pub fn data_dir() -> Result<PathBuf> {
    crate::platform::path_provider().data_dir()
}

/// 获取缓存目录
pub fn cache_dir() -> Result<PathBuf> {
    crate::platform::path_provider().cache_dir()
}

/// 获取临时目录
pub fn temp_dir() -> Result<PathBuf> {
    crate::platform::path_provider().temp_dir()
}

/// 获取文档目录
pub fn document_dir() -> Result<PathBuf> {
    crate::platform::path_provider().document_dir()
}

/// 获取外部存储应用数据目录
pub fn external_data_dir() -> Result<PathBuf> {
    crate::platform::path_provider().external_data_dir()
}

/// 获取外部存储缓存目录
pub fn external_cache_dir() -> Result<PathBuf> {
    crate::platform::path_provider().external_cache_dir()
}

// ========== 屏幕相关 ==========

/// 获取屏幕信息
pub fn screen_info() -> Result<ScreenInfo> {
    crate::platform::screen_provider().screen_info()
}

/// 获取屏幕宽度
pub fn screen_width() -> Result<u32> {
    crate::platform::screen_provider().screen_width()
}

/// 获取屏幕高度
pub fn screen_height() -> Result<u32> {
    crate::platform::screen_provider().screen_height()
}

/// 获取缩放因子
pub fn scale_factor() -> Result<f32> {
    crate::platform::screen_provider().scale_factor()
}

/// 获取屏幕方向
pub fn orientation() -> Result<Orientation> {
    crate::platform::screen_provider().orientation()
}

// ========== 异步接口 ==========

/// 异步获取平台信息
pub async fn platform_info_async() -> Result<PlatformInfo> {
    crate::platform::device_provider()
        .platform_info_async()
        .await
}

/// 异步获取操作系统版本
pub async fn os_version_async() -> Result<String> {
    crate::platform::device_provider().os_version_async().await
}

/// 异步获取设备型号
pub async fn device_model_async() -> Result<String> {
    crate::platform::device_provider()
        .device_model_async()
        .await
}

/// 异步获取应用数据目录
pub async fn data_dir_async() -> Result<PathBuf> {
    crate::platform::path_provider().data_dir_async().await
}

/// 异步获取缓存目录
pub async fn cache_dir_async() -> Result<PathBuf> {
    crate::platform::path_provider().cache_dir_async().await
}

/// 异步获取临时目录
pub async fn temp_dir_async() -> Result<PathBuf> {
    crate::platform::path_provider().temp_dir_async().await
}

/// 异步获取文档目录
pub async fn document_dir_async() -> Result<PathBuf> {
    crate::platform::path_provider().document_dir_async().await
}

/// 异步获取外部存储应用数据目录
pub async fn external_data_dir_async() -> Result<PathBuf> {
    crate::platform::path_provider()
        .external_data_dir_async()
        .await
}

/// 异步获取外部存储缓存目录
pub async fn external_cache_dir_async() -> Result<PathBuf> {
    crate::platform::path_provider()
        .external_cache_dir_async()
        .await
}

/// 异步获取屏幕信息
pub async fn screen_info_async() -> Result<ScreenInfo> {
    crate::platform::screen_provider().screen_info_async().await
}

/// 异步获取屏幕方向
pub async fn orientation_async() -> Result<Orientation> {
    crate::platform::screen_provider().orientation_async().await
}
