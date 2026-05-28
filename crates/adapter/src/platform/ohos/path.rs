use std::path::PathBuf;

use crate::error::Result;
use crate::traits::PathProvider;

pub struct OhosPathProvider;

impl PathProvider for OhosPathProvider {
    fn data_dir(&self) -> Result<PathBuf> {
        // OpenHarmony NDK: OH_AbilityRuntime_ApplicationContextGetFilesDir
        // 需要链接 libability_runtime.so
        #[cfg(target_os = "ohos")]
        {
            type AbilityRuntime_ErrorCode = i32;
            const ABILITY_RUNTIME_ERROR_CODE_NO_ERROR: i32 = 0;

            extern "C" {
                fn OH_AbilityRuntime_ApplicationContextGetFilesDir(
                    buffer: *mut std::ffi::c_char,
                    bufferSize: i32,
                    writeLength: *mut i32,
                ) -> AbilityRuntime_ErrorCode;
            }

            let mut buf = [0u8; 512];
            let mut write_length: i32 = 0;
            unsafe {
                let result = OH_AbilityRuntime_ApplicationContextGetFilesDir(
                    buf.as_mut_ptr() as *mut std::ffi::c_char,
                    buf.len() as i32,
                    &mut write_length,
                );
                if result == ABILITY_RUNTIME_ERROR_CODE_NO_ERROR && write_length > 0 {
                    let path = String::from_utf8_lossy(&buf[..write_length as usize]);
                    return Ok(PathBuf::from(path.to_string()));
                }
            }
            Err(crate::error::PlatformError::FfiError(
                "Failed to get files dir".to_string(),
            ))
        }
        #[cfg(not(target_os = "ohos"))]
        {
            Err(crate::error::PlatformError::NotSupported)
        }
    }

    fn cache_dir(&self) -> Result<PathBuf> {
        // OpenHarmony NDK: OH_AbilityRuntime_ApplicationContextGetCacheDir
        #[cfg(target_os = "ohos")]
        {
            type AbilityRuntime_ErrorCode = i32;
            const ABILITY_RUNTIME_ERROR_CODE_NO_ERROR: i32 = 0;

            extern "C" {
                fn OH_AbilityRuntime_ApplicationContextGetCacheDir(
                    buffer: *mut std::ffi::c_char,
                    bufferSize: i32,
                    writeLength: *mut i32,
                ) -> AbilityRuntime_ErrorCode;
            }

            let mut buf = [0u8; 512];
            let mut write_length: i32 = 0;
            unsafe {
                let result = OH_AbilityRuntime_ApplicationContextGetCacheDir(
                    buf.as_mut_ptr() as *mut std::ffi::c_char,
                    buf.len() as i32,
                    &mut write_length,
                );
                if result == ABILITY_RUNTIME_ERROR_CODE_NO_ERROR && write_length > 0 {
                    let path = String::from_utf8_lossy(&buf[..write_length as usize]);
                    return Ok(PathBuf::from(path.to_string()));
                }
            }
            Err(crate::error::PlatformError::FfiError(
                "Failed to get cache dir".to_string(),
            ))
        }
        #[cfg(not(target_os = "ohos"))]
        {
            Err(crate::error::PlatformError::NotSupported)
        }
    }

    fn temp_dir(&self) -> Result<PathBuf> {
        // OpenHarmony NDK: OH_AbilityRuntime_ApplicationContextGetTempDir (since API 16)
        #[cfg(target_os = "ohos")]
        {
            type AbilityRuntime_ErrorCode = i32;
            const ABILITY_RUNTIME_ERROR_CODE_NO_ERROR: i32 = 0;

            extern "C" {
                fn OH_AbilityRuntime_ApplicationContextGetTempDir(
                    buffer: *mut std::ffi::c_char,
                    bufferSize: i32,
                    writeLength: *mut i32,
                ) -> AbilityRuntime_ErrorCode;
            }

            let mut buf = [0u8; 512];
            let mut write_length: i32 = 0;
            unsafe {
                let result = OH_AbilityRuntime_ApplicationContextGetTempDir(
                    buf.as_mut_ptr() as *mut std::ffi::c_char,
                    buf.len() as i32,
                    &mut write_length,
                );
                if result == ABILITY_RUNTIME_ERROR_CODE_NO_ERROR && write_length > 0 {
                    let path = String::from_utf8_lossy(&buf[..write_length as usize]);
                    return Ok(PathBuf::from(path.to_string()));
                }
            }
            // fallback 到系统 temp 目录
            Ok(std::env::temp_dir())
        }
        #[cfg(not(target_os = "ohos"))]
        {
            Ok(std::env::temp_dir())
        }
    }

    fn document_dir(&self) -> Result<PathBuf> {
        // OpenHarmony: filesDir + "/documents"
        Ok(self.data_dir()?.join("documents"))
    }

    fn external_data_dir(&self) -> Result<PathBuf> {
        // OpenHarmony 无外部存储概念，返回与内部路径相同
        self.data_dir()
    }

    fn external_cache_dir(&self) -> Result<PathBuf> {
        // OpenHarmony 无外部存储概念，返回与内部路径相同
        self.cache_dir()
    }
}
