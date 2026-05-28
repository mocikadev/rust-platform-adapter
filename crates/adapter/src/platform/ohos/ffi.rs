//! OpenHarmony 平台 FFI 类型定义与函数声明
//!
//! 集中管理所有跨模块共享的 FFI 绑定，避免重复定义。

// ── AbilityRuntime (path 模块) ──────────────────────────────────────

pub type AbilityRuntime_ErrorCode = i32;
pub const ABILITY_RUNTIME_ERROR_CODE_NO_ERROR: i32 = 0;

extern "C" {
    pub fn OH_AbilityRuntime_ApplicationContextGetFilesDir(
        buffer: *mut std::ffi::c_char,
        bufferSize: i32,
        writeLength: *mut i32,
    ) -> AbilityRuntime_ErrorCode;

    pub fn OH_AbilityRuntime_ApplicationContextGetCacheDir(
        buffer: *mut std::ffi::c_char,
        bufferSize: i32,
        writeLength: *mut i32,
    ) -> AbilityRuntime_ErrorCode;

    pub fn OH_AbilityRuntime_ApplicationContextGetTempDir(
        buffer: *mut std::ffi::c_char,
        bufferSize: i32,
        writeLength: *mut i32,
    ) -> AbilityRuntime_ErrorCode;
}

// ── DisplayManager (screen 模块) ───────────────────────────────────

pub type NativeDisplayManager_ErrorCode = i32;
pub const DISPLAY_MANAGER_OK: i32 = 0;

/// NativeDisplayManager_Orientation 枚举
pub const DISPLAY_MANAGER_PORTRAIT: i32 = 0;
pub const DISPLAY_MANAGER_LANDSCAPE: i32 = 1;

extern "C" {
    pub fn OH_NativeDisplayManager_GetDefaultDisplayWidth(
        width: *mut i32,
    ) -> NativeDisplayManager_ErrorCode;

    pub fn OH_NativeDisplayManager_GetDefaultDisplayHeight(
        height: *mut i32,
    ) -> NativeDisplayManager_ErrorCode;

    pub fn OH_NativeDisplayManager_GetDefaultDisplayDensityDpi(
        dpi: *mut i32,
    ) -> NativeDisplayManager_ErrorCode;

    pub fn OH_NativeDisplayManager_GetDefaultDisplayVirtualPixelRatio(
        vpr: *mut f32,
    ) -> NativeDisplayManager_ErrorCode;

    pub fn OH_NativeDisplayManager_GetDefaultDisplayOrientation(
        orientation: *mut i32,
    ) -> NativeDisplayManager_ErrorCode;
}

// ── DeviceInfo (device 模块) ───────────────────────────────────────

extern "C" {
    pub fn OH_GetOSFullName() -> *const std::ffi::c_char;
    pub fn OH_GetProductModel() -> *const std::ffi::c_char;
    pub fn OH_GetDeviceType() -> *const std::ffi::c_char;
}
