use crate::error::Result;
use crate::traits::DeviceInfo;
use crate::types::{CpuArch, DeviceForm, OsType, PlatformInfo, CURRENT_ARCH};

use super::ffi::*;

pub struct OhosDeviceInfo;

impl DeviceInfo for OhosDeviceInfo {
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
        OsType::Ohos
    }

    fn os_version(&self) -> Result<String> {
        // OpenHarmony 使用 deviceinfo.h 官方 API
        // OH_GetOSFullName() 获取 OS 完整版本
        #[cfg(target_os = "ohos")]
        {
            // Safety: OH_GetOSFullName 是 OpenHarmony deviceinfo NDK 提供的线程安全函数，
            // 返回指向静态字符串的指针，调用者不需要释放，使用前检查非 null
            unsafe {
                let ptr = OH_GetOSFullName();
                if !ptr.is_null() {
                    let c_str = std::ffi::CStr::from_ptr(ptr);
                    return Ok(c_str.to_string_lossy().to_string());
                }
            }
            Err(crate::error::PlatformError::FfiError(
                "Failed to get OS version".to_string(),
            ))
        }
        #[cfg(not(target_os = "ohos"))]
        {
            Err(crate::error::PlatformError::NotSupported)
        }
    }

    fn device_model(&self) -> Result<String> {
        // OpenHarmony 使用 deviceinfo.h 官方 API
        // OH_GetProductModel() 获取产品型号
        #[cfg(target_os = "ohos")]
        {
            // Safety: OH_GetProductModel 是 OpenHarmony deviceinfo NDK 提供的线程安全函数，
            // 返回指向静态字符串的指针，调用者不需要释放，使用前检查非 null
            unsafe {
                let ptr = OH_GetProductModel();
                if !ptr.is_null() {
                    let c_str = std::ffi::CStr::from_ptr(ptr);
                    return Ok(c_str.to_string_lossy().to_string());
                }
            }
            Err(crate::error::PlatformError::FfiError(
                "Failed to get device model".to_string(),
            ))
        }
        #[cfg(not(target_os = "ohos"))]
        {
            Err(crate::error::PlatformError::NotSupported)
        }
    }

    fn cpu_arch(&self) -> CpuArch {
        CURRENT_ARCH
    }

    fn device_form(&self) -> DeviceForm {
        // OpenHarmony 使用 deviceinfo.h 官方 API
        // OH_GetDeviceType() 获取设备类型
        // 返回值: "phone"/"default"(手机), "tablet"(平板), "tv"(电视),
        //         "wearable"/"liteWearable"(穿戴), "car"(车载), "smartVision"(视觉)
        #[cfg(target_os = "ohos")]
        {
            // Safety: OH_GetDeviceType 是 OpenHarmony deviceinfo NDK 提供的线程安全函数，
            // 返回指向静态字符串的指针，调用者不需要释放，使用前检查非 null
            unsafe {
                let ptr = OH_GetDeviceType();
                if !ptr.is_null() {
                    let c_str = std::ffi::CStr::from_ptr(ptr);
                    let device_type = c_str.to_string_lossy();
                    return match device_type.as_ref() {
                        "phone" | "default" => DeviceForm::Phone,
                        "tablet" => DeviceForm::Tablet,
                        "tv" => DeviceForm::Tv,
                        "wearable" | "liteWearable" => DeviceForm::Wearable,
                        "car" => DeviceForm::Car,
                        "2in1" => DeviceForm::Desktop,
                        "smartVision" => DeviceForm::IoT,
                        _ => DeviceForm::Unknown,
                    };
                }
            }
            DeviceForm::Unknown
        }
        #[cfg(not(target_os = "ohos"))]
        {
            DeviceForm::Unknown
        }
    }
}
