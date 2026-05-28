mod device;
mod path;
mod screen;

use crate::traits::{DeviceInfo, PathProvider, ScreenProvider};

pub use device::MacosDeviceInfo;
pub use path::MacosPathProvider;
pub use screen::MacosScreenProvider;

/// 获取设备信息提供者
pub fn device_provider() -> impl DeviceInfo {
    MacosDeviceInfo
}

/// 获取路径提供者
pub fn path_provider() -> impl PathProvider {
    MacosPathProvider
}

/// 获取屏幕提供者
pub fn screen_provider() -> impl ScreenProvider {
    MacosScreenProvider
}
