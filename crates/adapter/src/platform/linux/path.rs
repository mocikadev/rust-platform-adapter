use std::path::PathBuf;

use crate::error::Result;
use crate::traits::PathProvider;

pub struct LinuxPathProvider;

impl PathProvider for LinuxPathProvider {
    fn data_dir(&self) -> Result<PathBuf> {
        dirs::data_dir().ok_or(crate::error::PlatformError::NotSupported)
    }

    fn cache_dir(&self) -> Result<PathBuf> {
        dirs::cache_dir().ok_or(crate::error::PlatformError::NotSupported)
    }

    fn temp_dir(&self) -> Result<PathBuf> {
        Ok(std::env::temp_dir())
    }

    fn document_dir(&self) -> Result<PathBuf> {
        dirs::document_dir().ok_or(crate::error::PlatformError::NotSupported)
    }

    fn external_data_dir(&self) -> Result<PathBuf> {
        self.data_dir()
    }

    fn external_cache_dir(&self) -> Result<PathBuf> {
        self.cache_dir()
    }
}
