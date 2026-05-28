use crate::error::{PlatformError, Result};
use crate::traits::DeviceInfo;
use crate::types::{CpuArch, DeviceForm, OsType, PlatformInfo, CURRENT_ARCH};

pub struct WindowsDeviceInfo;

impl DeviceInfo for WindowsDeviceInfo {
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
        OsType::Windows
    }

    fn os_version(&self) -> Result<String> {
        use sysinfo::System;
        Ok(System::os_version().unwrap_or_else(|| "Unknown".to_string()))
    }

    fn device_model(&self) -> Result<String> {
        #[cfg(target_os = "windows")]
        {
            use windows::core::HSTRING;
            use windows::Win32::System::Registry::{
                RegCloseKey, RegGetValueW, RegOpenKeyExW, HKEY_LOCAL_MACHINE, KEY_READ,
                RRF_RT_REG_SZ,
            };

            let subkey = HSTRING::from(r"HARDWARE\DESCRIPTION\System\BIOS");
            let value_name = HSTRING::from("SystemProductName");

            let mut hkey = Default::default();
            let result =
                unsafe { RegOpenKeyExW(HKEY_LOCAL_MACHINE, &subkey, None, KEY_READ, &mut hkey) };

            if result.is_err() {
                return Err(PlatformError::FfiError(
                    "Failed to open registry key for device model".to_string(),
                ));
            }

            let mut buf_size: u32 = 512;
            let mut buf = vec![0u16; 256];

            let result = unsafe {
                RegGetValueW(
                    hkey,
                    None,
                    &value_name,
                    RRF_RT_REG_SZ,
                    None,
                    Some(buf.as_mut_ptr() as *mut _),
                    Some(&mut buf_size),
                )
            };

            let _ = unsafe { RegCloseKey(hkey) };

            if result.is_err() {
                return Err(PlatformError::FfiError(
                    "Failed to read SystemProductName from registry".to_string(),
                ));
            }

            let len = (buf_size / 2) as usize;
            let model = String::from_utf16_lossy(&buf[..len.saturating_sub(1)]);
            let model = model.trim().to_string();

            if model.is_empty() {
                use sysinfo::System;
                return System::host_name().ok_or_else(|| {
                    PlatformError::FfiError("Failed to get device model".to_string())
                });
            }

            Ok(model)
        }
        #[cfg(not(target_os = "windows"))]
        {
            Err(PlatformError::NotSupported)
        }
    }

    fn cpu_arch(&self) -> CpuArch {
        CURRENT_ARCH
    }

    fn device_form(&self) -> DeviceForm {
        DeviceForm::Desktop
    }
}
