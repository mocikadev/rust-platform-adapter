use std::path::PathBuf;

use crate::error::Result;
use crate::traits::PathProvider;

pub struct MacosPathProvider;

impl PathProvider for MacosPathProvider {
    fn data_dir(&self) -> Result<PathBuf> {
        // macOS: ~/Library/Application Support
        dirs::data_dir()
            .ok_or_else(|| crate::error::PlatformError::NotSupported)
    }

    fn cache_dir(&self) -> Result<PathBuf> {
        // macOS: ~/Library/Caches
        dirs::cache_dir()
            .ok_or_else(|| crate::error::PlatformError::NotSupported)
    }

    fn temp_dir(&self) -> Result<PathBuf> {
        Ok(std::env::temp_dir())
    }

    fn document_dir(&self) -> Result<PathBuf> {
        // macOS: ~/Documents
        dirs::document_dir()
            .ok_or_else(|| crate::error::PlatformError::NotSupported)
    }
}
