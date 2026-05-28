mod device;
mod path;
mod screen;

use crate::traits::{DeviceInfo, PathProvider, ScreenProvider};

pub use device::WindowsDeviceInfo;
pub use path::WindowsPathProvider;
pub use screen::WindowsScreenProvider;

/// 获取设备信息提供者
pub fn device_provider() -> impl DeviceInfo {
    WindowsDeviceInfo
}

/// 获取路径提供者
pub fn path_provider() -> impl PathProvider {
    WindowsPathProvider
}

/// 获取屏幕提供者
pub fn screen_provider() -> impl ScreenProvider {
    WindowsScreenProvider
}
