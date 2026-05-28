/// 操作系统类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OsType {
    Android,
    Ios,
    /// OpenHarmony / HarmonyOS (target_os = "ohos")
    Ohos,
    Windows,
    Linux,
    MacOS,
}

impl OsType {
    /// 获取当前编译目标的操作系统类型（编译时常量）
    pub const fn current() -> Self {
        #[cfg(target_os = "android")]
        {
            OsType::Android
        }
        #[cfg(target_os = "ios")]
        {
            OsType::Ios
        }
        #[cfg(target_os = "ohos")]
        {
            OsType::Ohos
        }
        #[cfg(target_os = "windows")]
        {
            OsType::Windows
        }
        #[cfg(target_os = "linux")]
        {
            OsType::Linux
        }
        #[cfg(target_os = "macos")]
        {
            OsType::MacOS
        }
        #[cfg(not(any(
            target_os = "android",
            target_os = "ios",
            target_os = "ohos",
            target_os = "windows",
            target_os = "linux",
            target_os = "macos"
        )))]
        {
            compile_error!("Unsupported target OS")
        }
    }

    pub const fn is_android(&self) -> bool {
        matches!(self, OsType::Android)
    }
    pub const fn is_ios(&self) -> bool {
        matches!(self, OsType::Ios)
    }
    pub const fn is_ohos(&self) -> bool {
        matches!(self, OsType::Ohos)
    }
    pub const fn is_windows(&self) -> bool {
        matches!(self, OsType::Windows)
    }
    pub const fn is_linux(&self) -> bool {
        matches!(self, OsType::Linux)
    }
    pub const fn is_macos(&self) -> bool {
        matches!(self, OsType::MacOS)
    }

    /// 是否为移动端（Android / iOS）
    /// 注意：OpenHarmony 支持多种设备形态，不在此判断
    pub const fn is_mobile(&self) -> bool {
        matches!(self, OsType::Android | OsType::Ios)
    }

    /// 是否为桌面端（Windows / Linux / macOS）
    /// 注意：OpenHarmony 也支持 PC，不在此判断
    pub const fn is_desktop(&self) -> bool {
        matches!(self, OsType::Windows | OsType::Linux | OsType::MacOS)
    }
}

/// 设备形态类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceForm {
    Phone,
    Tablet,
    Desktop,
    Tv,
    Car,
    Wearable,
    IoT,
    Unknown,
}

impl DeviceForm {
    /// 获取当前设备形态（运行时检测）
    pub fn current() -> Self {
        use crate::traits::DeviceInfo;
        crate::platform::device_provider().device_form()
    }

    pub const fn is_phone(&self) -> bool {
        matches!(self, DeviceForm::Phone)
    }
    pub const fn is_tablet(&self) -> bool {
        matches!(self, DeviceForm::Tablet)
    }
    pub const fn is_desktop(&self) -> bool {
        matches!(self, DeviceForm::Desktop)
    }
    pub const fn is_tv(&self) -> bool {
        matches!(self, DeviceForm::Tv)
    }
    pub const fn is_car(&self) -> bool {
        matches!(self, DeviceForm::Car)
    }
    pub const fn is_wearable(&self) -> bool {
        matches!(self, DeviceForm::Wearable)
    }
    pub const fn is_iot(&self) -> bool {
        matches!(self, DeviceForm::IoT)
    }

    /// 是否为移动端（手机/平板）
    pub const fn is_mobile(&self) -> bool {
        matches!(self, DeviceForm::Phone | DeviceForm::Tablet)
    }
}

/// CPU 架构
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CpuArch {
    X86,
    X86_64,
    Arm,
    Arm64,
    RiscV32,
    RiscV64,
    Unknown,
}

impl CpuArch {
    /// 获取当前编译目标的 CPU 架构（编译时常量）
    pub const fn current() -> Self {
        #[cfg(target_arch = "x86")]
        {
            CpuArch::X86
        }
        #[cfg(target_arch = "x86_64")]
        {
            CpuArch::X86_64
        }
        #[cfg(target_arch = "arm")]
        {
            CpuArch::Arm
        }
        #[cfg(target_arch = "aarch64")]
        {
            CpuArch::Arm64
        }
        #[cfg(target_arch = "riscv32")]
        {
            CpuArch::RiscV32
        }
        #[cfg(target_arch = "riscv64")]
        {
            CpuArch::RiscV64
        }
        #[cfg(not(any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "arm",
            target_arch = "aarch64",
            target_arch = "riscv32",
            target_arch = "riscv64"
        )))]
        {
            CpuArch::Unknown
        }
    }
}

/// 平台信息
#[derive(Debug, Clone, PartialEq)]
pub struct PlatformInfo {
    pub os_type: OsType,
    pub os_version: String,
    pub device_model: String,
    pub cpu_arch: CpuArch,
    pub device_form: DeviceForm,
}

/// 屏幕方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Portrait,
    Landscape,
    Unknown,
}

/// 屏幕信息
#[derive(Debug, Clone, PartialEq)]
pub struct ScreenInfo {
    pub width: u32,
    pub height: u32,
    pub dpi: f32,
    pub scale_factor: f32,
    pub orientation: Orientation,
}

/// 编译时常量：当前操作系统类型
pub const CURRENT_OS: OsType = OsType::current();

/// 编译时常量：当前 CPU 架构
pub const CURRENT_ARCH: CpuArch = CpuArch::current();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_type_equality() {
        assert_eq!(OsType::Linux, OsType::Linux);
        assert_ne!(OsType::Linux, OsType::Windows);
    }

    #[test]
    fn test_os_type_hash() {
        use std::collections::HashSet;
        let set: HashSet<OsType> = [OsType::Linux, OsType::Windows, OsType::Linux].into();
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_os_type_is_methods() {
        assert!(OsType::Android.is_android());
        assert!(OsType::Android.is_mobile());
        assert!(!OsType::Android.is_desktop());

        assert!(OsType::Linux.is_linux());
        assert!(OsType::Linux.is_desktop());
        assert!(!OsType::Linux.is_mobile());

        assert!(!OsType::Ohos.is_mobile());
        assert!(!OsType::Ohos.is_desktop());
    }

    #[test]
    fn test_device_form_is_methods() {
        assert!(DeviceForm::Phone.is_phone());
        assert!(DeviceForm::Phone.is_mobile());
        assert!(!DeviceForm::Phone.is_desktop());

        assert!(DeviceForm::Tablet.is_tablet());
        assert!(DeviceForm::Tablet.is_mobile());

        assert!(DeviceForm::Desktop.is_desktop());
        assert!(!DeviceForm::Desktop.is_mobile());

        assert!(DeviceForm::Tv.is_tv());
        assert!(DeviceForm::Car.is_car());
        assert!(DeviceForm::Wearable.is_wearable());
        assert!(DeviceForm::IoT.is_iot());
    }

    #[test]
    fn test_device_form_hash() {
        use std::collections::HashSet;
        let set: HashSet<DeviceForm> =
            [DeviceForm::Phone, DeviceForm::Desktop, DeviceForm::Phone].into();
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_cpu_arch_values() {
        assert_ne!(CpuArch::X86, CpuArch::X86_64);
        assert_ne!(CpuArch::Arm, CpuArch::Arm64);
        assert_ne!(CpuArch::RiscV32, CpuArch::RiscV64);
    }

    #[test]
    fn test_cpu_arch_current() {
        // 在当前编译环境下应该返回有效架构
        let arch = CpuArch::current();
        assert_ne!(arch, CpuArch::Unknown);
    }

    #[test]
    fn test_orientation_values() {
        assert_ne!(Orientation::Portrait, Orientation::Landscape);
        assert_ne!(Orientation::Landscape, Orientation::Unknown);
    }

    #[test]
    fn test_screen_info_partial_eq() {
        let a = ScreenInfo {
            width: 1920,
            height: 1080,
            dpi: 96.0,
            scale_factor: 1.0,
            orientation: Orientation::Landscape,
        };
        let b = ScreenInfo {
            width: 1920,
            height: 1080,
            dpi: 96.0,
            scale_factor: 1.0,
            orientation: Orientation::Landscape,
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_platform_info_partial_eq() {
        let a = PlatformInfo {
            os_type: OsType::Linux,
            os_version: "1.0".to_string(),
            device_model: "Test".to_string(),
            cpu_arch: CpuArch::X86_64,
            device_form: DeviceForm::Desktop,
        };
        let b = PlatformInfo {
            os_type: OsType::Linux,
            os_version: "1.0".to_string(),
            device_model: "Test".to_string(),
            cpu_arch: CpuArch::X86_64,
            device_form: DeviceForm::Desktop,
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_current_os_constant() {
        assert_eq!(CURRENT_OS, OsType::current());
    }

    #[test]
    fn test_current_arch_constant() {
        assert_eq!(CURRENT_ARCH, CpuArch::current());
    }
}
