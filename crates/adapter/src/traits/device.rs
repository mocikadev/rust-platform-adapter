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
}
