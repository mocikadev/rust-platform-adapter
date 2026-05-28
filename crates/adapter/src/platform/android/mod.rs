mod device;
mod path;
mod screen;

use crate::traits::{DeviceInfo, PathProvider, ScreenProvider};

pub use device::AndroidDeviceInfo;
pub use path::AndroidPathProvider;
pub use screen::AndroidScreenProvider;

/// 获取设备信息提供者
pub fn device_provider() -> impl DeviceInfo {
    AndroidDeviceInfo
}

/// 获取路径提供者
pub fn path_provider() -> impl PathProvider {
    AndroidPathProvider
}

/// 获取屏幕提供者
pub fn screen_provider() -> impl ScreenProvider {
    AndroidScreenProvider
}
