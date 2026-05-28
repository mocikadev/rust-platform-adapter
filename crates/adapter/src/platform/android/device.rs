use crate::error::Result;
use crate::traits::DeviceInfo;
use crate::types::{CpuArch, DeviceForm, OsType, PlatformInfo, CURRENT_ARCH};

pub struct AndroidDeviceInfo;

impl DeviceInfo for AndroidDeviceInfo {
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
        OsType::Android
    }

    fn os_version(&self) -> Result<String> {
        // Android 使用 NDK API 获取系统版本
        #[cfg(target_os = "android")]
        {
            // 使用 libc 的 __system_property_get 获取 ro.build.version.release
            extern "C" {
                fn __system_property_get(
                    name: *const std::ffi::c_char,
                    value: *mut std::ffi::c_char,
                ) -> i32;
                fn android_get_device_api_level() -> i32;
            }

            let mut buf = [0u8; 256];
            unsafe {
                let len = __system_property_get(
                    b"ro.build.version.release\0".as_ptr() as *const std::ffi::c_char,
                    buf.as_mut_ptr() as *mut std::ffi::c_char,
                );
                if len > 0 {
                    return Ok(String::from_utf8_lossy(&buf[..len as usize]).to_string());
                }
                // fallback 到 SDK 版本
                let api_level = android_get_device_api_level();
                Ok(format!("Android API {}", api_level))
            }
        }
        #[cfg(not(target_os = "android"))]
        {
            Err(crate::error::PlatformError::NotSupported)
        }
    }

    fn device_model(&self) -> Result<String> {
        // Android 使用 __system_property_get 获取设备型号
        #[cfg(target_os = "android")]
        {
            extern "C" {
                fn __system_property_get(
                    name: *const std::ffi::c_char,
                    value: *mut std::ffi::c_char,
                ) -> i32;
            }

            let mut buf = [0u8; 256];
            unsafe {
                let len = __system_property_get(
                    b"ro.product.model\0".as_ptr() as *const _,
                    buf.as_mut_ptr() as *mut _,
                );
                if len > 0 {
                    return Ok(String::from_utf8_lossy(&buf[..len as usize]).to_string());
                }
            }
            Err(crate::error::PlatformError::FfiError(
                "Failed to get device model".to_string(),
            ))
        }
        #[cfg(not(target_os = "android"))]
        {
            Err(crate::error::PlatformError::NotSupported)
        }
    }

    fn cpu_arch(&self) -> CpuArch {
        CURRENT_ARCH
    }

    fn device_form(&self) -> DeviceForm {
        // Android 使用 NDK Configuration 判断设备类型
        #[cfg(target_os = "android")]
        {
            use ndk::configuration::{Configuration, ScreenSize};
            let config = Configuration::new();
            let screen_size = config.screen_size();

            match screen_size {
                ScreenSize::Small | ScreenSize::Normal => DeviceForm::Phone,
                ScreenSize::Large | ScreenSize::XLarge => DeviceForm::Tablet,
                _ => DeviceForm::Unknown,
            }
        }
        #[cfg(not(target_os = "android"))]
        {
            DeviceForm::Unknown
        }
    }
}
