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
            // Safety: libc::sysctlbyname 第一次调用传入 null 输出缓冲区和 &mut size，
            // 仅用于获取所需缓冲区大小，oldp 为 null 时内核不写入数据，newp 为 null 无修改操作
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
                // Safety: libc::sysctlbyname 第二次调用，name 指向以 null 结尾的合法字节串 "hw.model"，
                // buf 是根据第一次调用返回的 size 分配的足够大小的缓冲区，
                // newp 为 null 不修改内核参数，函数返回 0 表示成功
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
        // sysctl 获取 hw.model 失败时返回错误，不回退到 hostname
        Err(crate::error::PlatformError::FfiError(
            "Failed to get device model via sysctl hw.model".to_string(),
        ))
    }

    fn cpu_arch(&self) -> CpuArch {
        CURRENT_ARCH
    }

    fn device_form(&self) -> DeviceForm {
        DeviceForm::Desktop
    }
}
