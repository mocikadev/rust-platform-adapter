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
        use sysinfo::System;
        Ok(System::os_version().unwrap_or_else(|| "Unknown".to_string()))
    }

    fn device_model(&self) -> Result<String> {
        // 通过 sysctl 获取 hw.model 得到真实设备型号（如 "MacBookPro18,3"）
        #[cfg(target_os = "macos")]
        {
            let mut size = 0;
            unsafe {
                // 第一次调用获取所需缓冲区大小
                libc::sysctlbyname(
                    b"hw.model\0".as_ptr() as *const i8,
                    std::ptr::null_mut(),
                    &mut size,
                    std::ptr::null_mut(),
                    0,
                );
            }
            if size > 0 {
                let mut buf = vec![0u8; size as usize];
                let result = unsafe {
                    libc::sysctlbyname(
                        b"hw.model\0".as_ptr() as *const i8,
                        buf.as_mut_ptr() as *mut libc::c_void,
                        &mut size,
                        std::ptr::null_mut(),
                        0,
                    )
                };
                if result == 0 {
                    // 去除末尾的 null 字节
                    let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
                    return Ok(String::from_utf8_lossy(&buf[..len]).to_string());
                }
            }
        }
        // Fallback
        use sysinfo::System;
        System::host_name().ok_or_else(|| {
            crate::error::PlatformError::FfiError("Failed to get device model".to_string())
        })
    }

    fn cpu_arch(&self) -> CpuArch {
        CURRENT_ARCH
    }

    fn device_form(&self) -> DeviceForm {
        DeviceForm::Desktop
    }
}
