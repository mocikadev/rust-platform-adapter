/// 操作系统类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuArch {
    X86,
    X86_64,
    Arm,
    Arm64,
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
        #[cfg(not(any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "arm",
            target_arch = "aarch64"
        )))]
        {
            CpuArch::Unknown
        }
    }
}

/// 平台信息
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
