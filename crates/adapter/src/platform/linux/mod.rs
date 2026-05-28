mod device;
mod path;
mod screen;

use crate::traits::{DeviceInfo, PathProvider, ScreenProvider};

pub use device::LinuxDeviceInfo;
pub use path::LinuxPathProvider;
pub use screen::LinuxScreenProvider;

/// 获取设备信息提供者
pub fn device_provider() -> impl DeviceInfo {
    LinuxDeviceInfo
}

/// 获取路径提供者
pub fn path_provider() -> impl PathProvider {
    LinuxPathProvider
}

/// 获取屏幕提供者
pub fn screen_provider() -> impl ScreenProvider {
    LinuxScreenProvider
}
