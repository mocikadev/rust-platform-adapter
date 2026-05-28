use std::path::PathBuf;

use crate::error::Result;
use crate::traits::PathProvider;

pub struct IosPathProvider;

impl PathProvider for IosPathProvider {
    fn data_dir(&self) -> Result<PathBuf> {
        // iOS: ~/Documents
        // 使用 NSSearchPathForDirectoriesInDomains 获取
        #[cfg(target_os = "ios")]
        {
            use objc2_foundation::{NSSearchPathDirectory, NSSearchPathDomainMask};
            let paths = unsafe {
                objc2_foundation::NSSearchPathForDirectoriesInDomains(
                    NSSearchPathDirectory::DocumentDirectory,
                    NSSearchPathDomainMask::UserDomainMask,
                    true, // expandTilde
                )
            };
            if let Some(path) = paths.first() {
                Ok(PathBuf::from(path.to_string()))
            } else {
                Err(crate::error::PlatformError::NotSupported)
            }
        }
        #[cfg(not(target_os = "ios"))]
        {
            Err(crate::error::PlatformError::NotSupported)
        }
    }

    fn cache_dir(&self) -> Result<PathBuf> {
        // iOS: ~/Library/Caches
        #[cfg(target_os = "ios")]
        {
            use objc2_foundation::{NSSearchPathDirectory, NSSearchPathDomainMask};
            let paths = unsafe {
                objc2_foundation::NSSearchPathForDirectoriesInDomains(
                    NSSearchPathDirectory::CachesDirectory,
                    NSSearchPathDomainMask::UserDomainMask,
                    true,
                )
            };
            if let Some(path) = paths.first() {
                Ok(PathBuf::from(path.to_string()))
            } else {
                Err(crate::error::PlatformError::NotSupported)
            }
        }
        #[cfg(not(target_os = "ios"))]
        {
            Err(crate::error::PlatformError::NotSupported)
        }
    }

    fn temp_dir(&self) -> Result<PathBuf> {
        // iOS: NSTemporaryDirectory()
        #[cfg(target_os = "ios")]
        {
            use objc2_foundation::NSTemporaryDirectory;
            let temp = unsafe { NSTemporaryDirectory() };
            Ok(PathBuf::from(temp.to_string()))
        }
        #[cfg(not(target_os = "ios"))]
        {
            Ok(std::env::temp_dir())
        }
    }

    fn document_dir(&self) -> Result<PathBuf> {
        // iOS: ~/Documents (与 data_dir 相同)
        self.data_dir()
    }
}
