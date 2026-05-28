use std::path::PathBuf;

use crate::error::Result;
use crate::traits::PathProvider;

pub struct AndroidPathProvider;

impl PathProvider for AndroidPathProvider {
    fn data_dir(&self) -> Result<PathBuf> {
        // Android: /data/data/<package>/files
        // 通过 JNI 获取应用内部存储路径
        // v1.0 使用默认路径，后续可通过 JNI 获取实际路径
        #[cfg(target_os = "android")]
        {
            // 默认内部存储路径
            // 实际应用中应通过 JNI 调用 Context.getFilesDir()
            Ok(PathBuf::from("/data/data/files"))
        }
        #[cfg(not(target_os = "android"))]
        {
            Err(crate::error::PlatformError::NotSupported)
        }
    }

    fn cache_dir(&self) -> Result<PathBuf> {
        // Android: /data/data/<package>/cache
        #[cfg(target_os = "android")]
        {
            Ok(PathBuf::from("/data/data/cache"))
        }
        #[cfg(not(target_os = "android"))]
        {
            Err(crate::error::PlatformError::NotSupported)
        }
    }

    fn temp_dir(&self) -> Result<PathBuf> {
        Ok(std::env::temp_dir())
    }

    fn document_dir(&self) -> Result<PathBuf> {
        // Android: /data/data/<package>/files/Documents
        Ok(self.data_dir()?.join("Documents"))
    }
}
