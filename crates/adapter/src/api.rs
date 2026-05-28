use std::path::PathBuf;

use crate::error::Result;
use crate::traits::{DeviceInfo, PathProvider, ScreenProvider};
use crate::types::{CpuArch, DeviceForm, Orientation, OsType, PlatformInfo, ScreenInfo, CURRENT_OS};

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
