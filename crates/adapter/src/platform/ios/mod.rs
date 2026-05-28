mod device;
mod path;
mod screen;

use crate::traits::{DeviceInfo, PathProvider, ScreenProvider};

pub use device::IosDeviceInfo;
pub use path::IosPathProvider;
pub use screen::IosScreenProvider;

/// 获取设备信息提供者
pub fn device_provider() -> impl DeviceInfo {
    IosDeviceInfo
}

/// 获取路径提供者
pub fn path_provider() -> impl PathProvider {
    IosPathProvider
}

/// 获取屏幕提供者
pub fn screen_provider() -> impl ScreenProvider {
    IosScreenProvider
}
