use std::path::PathBuf;

use crate::error::Result;
use crate::traits::PathProvider;

pub struct WindowsPathProvider;

impl PathProvider for WindowsPathProvider {
    fn data_dir(&self) -> Result<PathBuf> {
        // Windows: %APPDATA% (C:\Users\<user>\AppData\Roaming)
        dirs::data_dir()
            .ok_or_else(|| crate::error::PlatformError::NotSupported)
    }

    fn cache_dir(&self) -> Result<PathBuf> {
        // Windows: %LOCALAPPDATA% (C:\Users\<user>\AppData\Local)
        dirs::cache_dir()
            .ok_or_else(|| crate::error::PlatformError::NotSupported)
    }

    fn temp_dir(&self) -> Result<PathBuf> {
        Ok(std::env::temp_dir())
    }

    fn document_dir(&self) -> Result<PathBuf> {
        // Windows: %USERPROFILE%\Documents
        dirs::document_dir()
            .ok_or_else(|| crate::error::PlatformError::NotSupported)
    }
}
