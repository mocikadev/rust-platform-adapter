use sysinfo::System;

use crate::error::Result;
use crate::traits::DeviceInfo;
use crate::types::{CpuArch, DeviceForm, OsType, PlatformInfo, CURRENT_ARCH};

pub struct MacosDeviceInfo;

impl DeviceInfo for MacosDeviceInfo {
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
        OsType::MacOS
    }

    fn os_version(&self) -> Result<String> {
        Ok(System::os_version().unwrap_or_else(|| "Unknown".to_string()))
    }

    fn device_model(&self) -> Result<String> {
        // macOS 设备型号可通过 sysctl hw.model 获取
        // 或者使用 System::host_name() 作为 fallback
        System::host_name()
            .ok_or_else(|| crate::error::PlatformError::FfiError("Failed to get hostname".to_string()))
    }

    fn cpu_arch(&self) -> CpuArch {
        CURRENT_ARCH
    }

    fn device_form(&self) -> DeviceForm {
        DeviceForm::Desktop
    }
}
