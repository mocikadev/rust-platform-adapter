use std::future::Future;
use std::pin::Pin;

use crate::error::Result;
use crate::types::{CpuArch, DeviceForm, OsType, PlatformInfo};

/// 设备信息接口
pub trait DeviceInfo {
    /// 获取完整平台信息
    fn platform_info(&self) -> Result<PlatformInfo>;

    /// 获取操作系统类型
    fn os_type(&self) -> OsType;

    /// 获取操作系统版本
    fn os_version(&self) -> Result<String>;

    /// 获取设备型号
    fn device_model(&self) -> Result<String>;

    /// 获取 CPU 架构
    fn cpu_arch(&self) -> CpuArch;

    /// 获取设备形态
    fn device_form(&self) -> DeviceForm;

    // ===== 异步接口 =====

    /// 异步获取完整平台信息
    fn platform_info_async(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<PlatformInfo>> + Send + '_>> {
        Box::pin(std::future::ready(self.platform_info()))
    }

    /// 异步获取操作系统版本
    fn os_version_async(&self) -> Pin<Box<dyn Future<Output = Result<String>> + Send + '_>> {
        Box::pin(std::future::ready(self.os_version()))
    }

    /// 异步获取设备型号
    fn device_model_async(&self) -> Pin<Box<dyn Future<Output = Result<String>> + Send + '_>> {
        Box::pin(std::future::ready(self.device_model()))
    }
}
