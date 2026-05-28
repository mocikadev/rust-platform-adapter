use sysinfo::System;

use crate::error::Result;
use crate::traits::DeviceInfo;
use crate::types::{CpuArch, DeviceForm, OsType, PlatformInfo, CURRENT_ARCH};

pub struct LinuxDeviceInfo;

impl DeviceInfo for LinuxDeviceInfo {
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
        OsType::Linux
    }

    fn os_version(&self) -> Result<String> {
        Ok(System::os_version().unwrap_or_else(|| "Unknown".to_string()))
    }

    fn device_model(&self) -> Result<String> {
        // 尝试读取 /sys/class/dmi/id/product_name
        std::fs::read_to_string("/sys/class/dmi/id/product_name")
            .map(|s| s.trim().to_string())
            .or_else(|_| {
                // fallback 到主机名
                System::host_name()
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "hostname"))
            })
            .map_err(|e| crate::error::PlatformError::IoError(e))
    }

    fn cpu_arch(&self) -> CpuArch {
        CURRENT_ARCH
    }

    fn device_form(&self) -> DeviceForm {
        DeviceForm::Desktop
    }
}
