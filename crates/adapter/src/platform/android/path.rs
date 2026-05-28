use std::path::PathBuf;

use crate::error::{PlatformError, Result};
use crate::traits::PathProvider;

pub struct AndroidPathProvider;

impl AndroidPathProvider {
    /// 读取 /proc/self/cmdline 获取当前进程的包名
    ///
    /// Android 上每个应用的进程名就是其包名（如 com.example.app）。
    /// /proc/self/cmdline 是进程读取自身信息的标准 Linux 机制，
    /// Android SELinux 策略允许进程读取自己的 /proc/self/ 条目。
    #[cfg(target_os = "android")]
    fn get_package_name() -> Result<String> {
        let cmdline =
            std::fs::read_to_string("/proc/self/cmdline").map_err(PlatformError::IoError)?;
        let package = cmdline.split('\0').next().unwrap_or("").trim();
        if package.is_empty() {
            return Err(PlatformError::FfiError(
                "Failed to get package name from /proc/self/cmdline".to_string(),
            ));
        }
        Ok(package.to_string())
    }

    /// 获取应用内部数据根目录
    ///
    /// 优先使用 /data/data/<pkg>，如果不存在则尝试 /data/user/0/<pkg>（多用户场景）。
    /// 两者通常指向同一位置（符号链接），但某些厂商定制系统可能有差异。
    #[cfg(target_os = "android")]
    fn get_app_data_root() -> Result<PathBuf> {
        let pkg = Self::get_package_name()?;

        let primary = PathBuf::from(format!("/data/data/{}", pkg));
        if primary.exists() {
            return Ok(primary);
        }

        // 多用户场景 fallback
        let secondary = PathBuf::from(format!("/data/user/0/{}", pkg));
        if secondary.exists() {
            return Ok(secondary);
        }

        Ok(primary)
    }

    /// 获取应用外部存储根目录
    ///
    /// Android 外部存储应用专属目录：/sdcard/Android/data/<pkg>
    /// Android 4.4+ (API 19+) 无需权限即可访问此目录。
    /// 卸载应用后系统会删除此目录。
    #[cfg(target_os = "android")]
    fn get_app_external_root() -> Result<PathBuf> {
        let pkg = Self::get_package_name()?;

        // 标准外部存储路径
        let primary = PathBuf::from(format!("/sdcard/Android/data/{}", pkg));
        if primary.exists() {
            return Ok(primary);
        }

        // 某些设备使用 /storage/emulated/0/ 代替 /sdcard/
        let secondary = PathBuf::from(format!("/storage/emulated/0/Android/data/{}", pkg));
        if secondary.exists() {
            return Ok(secondary);
        }

        // 都不存在时返回 primary（让调用者决定如何处理）
        Ok(primary)
    }
}

impl PathProvider for AndroidPathProvider {
    fn data_dir(&self) -> Result<PathBuf> {
        #[cfg(target_os = "android")]
        {
            let root = Self::get_app_data_root()?;
            Ok(root.join("files"))
        }
        #[cfg(not(target_os = "android"))]
        {
            Err(PlatformError::NotSupported)
        }
    }

    fn cache_dir(&self) -> Result<PathBuf> {
        #[cfg(target_os = "android")]
        {
            let root = Self::get_app_data_root()?;
            Ok(root.join("cache"))
        }
        #[cfg(not(target_os = "android"))]
        {
            Err(PlatformError::NotSupported)
        }
    }

    fn temp_dir(&self) -> Result<PathBuf> {
        Ok(std::env::temp_dir())
    }

    fn document_dir(&self) -> Result<PathBuf> {
        // Android: <internal_data_root>/files/Documents
        Ok(self.data_dir()?.join("Documents"))
    }

    fn external_data_dir(&self) -> Result<PathBuf> {
        #[cfg(target_os = "android")]
        {
            let root = Self::get_app_external_root()?;
            Ok(root.join("files"))
        }
        #[cfg(not(target_os = "android"))]
        {
            Err(PlatformError::NotSupported)
        }
    }

    fn external_cache_dir(&self) -> Result<PathBuf> {
        #[cfg(target_os = "android")]
        {
            let root = Self::get_app_external_root()?;
            Ok(root.join("cache"))
        }
        #[cfg(not(target_os = "android"))]
        {
            Err(PlatformError::NotSupported)
        }
    }
}
