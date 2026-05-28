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
        #[cfg(target_os = "ios")]
        {
            use objc2::MainThreadMarker;
            use objc2_ui_kit::UIDevice;

            let mtm = MainThreadMarker::new().ok_or_else(|| {
                crate::error::PlatformError::FfiError("Not on main thread".to_string())
            })?;
            let device = UIDevice::currentDevice(mtm);
            let version = device.systemVersion();
            Ok(version.to_string())
        }
        #[cfg(not(target_os = "ios"))]
        {
            Err(crate::error::PlatformError::NotSupported)
        }
    }

    fn device_model(&self) -> Result<String> {
        #[cfg(target_os = "ios")]
        {
            use objc2::MainThreadMarker;
            use objc2_ui_kit::UIDevice;

            let mtm = MainThreadMarker::new().ok_or_else(|| {
                crate::error::PlatformError::FfiError("Not on main thread".to_string())
            })?;
            let device = UIDevice::currentDevice(mtm);
            let model = device.model();
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
        #[cfg(target_os = "ios")]
        {
            use objc2::MainThreadMarker;
            use objc2_ui_kit::{UIDevice, UIUserInterfaceIdiom};

            let mtm = match MainThreadMarker::new() {
                Some(mtm) => mtm,
                None => return DeviceForm::Unknown,
            };
            let device = UIDevice::currentDevice(mtm);
            let idiom = device.userInterfaceIdiom();
            match idiom {
                UIUserInterfaceIdiom::Phone => DeviceForm::Phone,
                UIUserInterfaceIdiom::Pad => DeviceForm::Tablet,
                UIUserInterfaceIdiom::TV => DeviceForm::Tv,
                UIUserInterfaceIdiom::CarPlay => DeviceForm::Car,
                _ => DeviceForm::Unknown,
            }
        }
        #[cfg(not(target_os = "ios"))]
        {
            DeviceForm::Unknown
        }
    }
}
