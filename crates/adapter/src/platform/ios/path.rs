use std::path::PathBuf;

use crate::error::Result;
use crate::traits::PathProvider;

pub struct IosPathProvider;

impl PathProvider for IosPathProvider {
    fn data_dir(&self) -> Result<PathBuf> {
        // iOS: ~/Library/Application Support
        // 应用数据目录：存放数据库、配置、KV 存储等不应被用户看到的数据
        // 注意：不同于 DocumentDirectory (~/Documents)，那是用户可见的文档目录
        #[cfg(target_os = "ios")]
        {
            use objc2_foundation::{NSSearchPathDirectory, NSSearchPathDomainMask};

            let paths = objc2_foundation::NSSearchPathForDirectoriesInDomains(
                NSSearchPathDirectory::ApplicationSupportDirectory,
                NSSearchPathDomainMask::UserDomainMask,
                true, // expandTilde
            );
            if let Some(path) = paths.firstObject() {
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

            let paths = objc2_foundation::NSSearchPathForDirectoriesInDomains(
                NSSearchPathDirectory::CachesDirectory,
                NSSearchPathDomainMask::UserDomainMask,
                true,
            );
            if let Some(path) = paths.firstObject() {
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
            let temp = NSTemporaryDirectory();
            Ok(PathBuf::from(temp.to_string()))
        }
        #[cfg(not(target_os = "ios"))]
        {
            Ok(std::env::temp_dir())
        }
    }

    fn document_dir(&self) -> Result<PathBuf> {
        // iOS: ~/Documents
        // 用户可见文档目录，会被 iTunes/iCloud 备份
        #[cfg(target_os = "ios")]
        {
            use objc2_foundation::{NSSearchPathDirectory, NSSearchPathDomainMask};

            let paths = objc2_foundation::NSSearchPathForDirectoriesInDomains(
                NSSearchPathDirectory::DocumentDirectory,
                NSSearchPathDomainMask::UserDomainMask,
                true,
            );
            if let Some(path) = paths.firstObject() {
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

    fn external_data_dir(&self) -> Result<PathBuf> {
        self.data_dir()
    }

    fn external_cache_dir(&self) -> Result<PathBuf> {
        self.cache_dir()
    }
}
