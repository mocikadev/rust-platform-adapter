use crate::error::Result;
use crate::traits::DeviceInfo;
use crate::types::{CpuArch, DeviceForm, OsType, PlatformInfo, CURRENT_ARCH};

pub struct IosDeviceInfo;

impl DeviceInfo for IosDeviceInfo {
    fn platform_info(&self) -> Result<PlatformInfo> {
        Ok(PlatformInfo {
            os_type: self.os_type(),
            os_version: self.os_version()?,
            device_model: self.device_model()?,
            cpu_arch: self.cpu_arch(),
            device_form: self.device_form(),
        })
    }

    fn os_type(&self) -> OsType {
        OsType::Ios
    }

    fn os_version(&self) -> Result<String> {
        // iOS 使用 objc2 获取系统版本
        // 需要 objc2-ui-kit 和 objc2-foundation
        #[cfg(target_os = "ios")]
        {
            use objc2_ui_kit::UIDevice;
            let device = unsafe { UIDevice::currentDevice() };
            let version = unsafe { device.systemVersion() };
            Ok(version.to_string())
        }
        #[cfg(not(target_os = "ios"))]
        {
            Err(crate::error::PlatformError::NotSupported)
        }
    }

    fn device_model(&self) -> Result<String> {
        // iOS 设备型号
        #[cfg(target_os = "ios")]
        {
            use objc2_ui_kit::UIDevice;
            let device = unsafe { UIDevice::currentDevice() };
            let model = unsafe { device.model() };
            Ok(model.to_string())
        }
        #[cfg(not(target_os = "ios"))]
        {
            Err(crate::error::PlatformError::NotSupported)
        }
    }

    fn cpu_arch(&self) -> CpuArch {
        CURRENT_ARCH
    }

    fn device_form(&self) -> DeviceForm {
        // iOS 使用 UIDevice.userInterfaceIdiom 判断设备类型
        #[cfg(target_os = "ios")]
        {
            use objc2_ui_kit::UIDevice;
            let device = unsafe { UIDevice::currentDevice() };
            let idiom = unsafe { device.userInterfaceIdiom() };
            match idiom {
                0 => DeviceForm::Phone,  // UIUserInterfaceIdiomPhone
                1 => DeviceForm::Tablet, // UIUserInterfaceIdiomPad
                _ => DeviceForm::Unknown,
            }
        }
        #[cfg(not(target_os = "ios"))]
        {
            DeviceForm::Unknown
        }
    }
}
