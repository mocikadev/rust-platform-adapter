mod device;
pub mod ffi;
mod path;
mod screen;

use crate::traits::{DeviceInfo, PathProvider, ScreenProvider};

pub use device::OhosDeviceInfo;
pub use path::OhosPathProvider;
pub use screen::OhosScreenProvider;

/// 获取设备信息提供者
pub fn device_provider() -> impl DeviceInfo {
    OhosDeviceInfo
}

/// 获取路径提供者
pub fn path_provider() -> impl PathProvider {
    OhosPathProvider
}

/// 获取屏幕提供者
pub fn screen_provider() -> impl ScreenProvider {
    OhosScreenProvider
}
