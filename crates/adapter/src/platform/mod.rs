#[cfg(target_os = "android")]
pub mod android;

#[cfg(target_os = "ios")]
pub mod ios;

#[cfg(target_os = "ohos")]
pub mod ohos;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

// 重导出当前平台实现
#[cfg(target_os = "android")]
pub use android::*;

#[cfg(target_os = "ios")]
pub use ios::*;

#[cfg(target_os = "ohos")]
pub use ohos::*;

#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "macos")]
pub use macos::*;
