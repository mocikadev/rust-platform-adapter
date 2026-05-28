# API 接口设计

## 设计原则

1. **Trait 驱动**：每个功能模块定义一个 trait，平台无关
2. **零开销抽象**：编译期静态分发，无运行时开销
3. **统一错误处理**：所有接口返回 `Result<T>`
4. **同步优先**：v1.0 以同步接口为主，异步按需扩展

---

## 一、公共类型

### types.rs

```rust
/// 操作系统类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsType {
    Android,
    Ios,
    /// OpenHarmony / HarmonyOS (target_os = "ohos")
    /// OpenHarmony 是开源基础，HarmonyOS 是华为商业实现
    /// 支持多种设备：手机、平板、PC、IoT、穿戴设备等
    Ohos,
    Windows,
    Linux,
    MacOS,
}

/// 设备形态类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceForm {
    /// 手机
    Phone,
    /// 平板
    Tablet,
    /// PC / 桌面电脑
    Desktop,
    /// 电视
    Tv,
    /// 车载系统
    Car,
    /// 穿戴设备（手表、手环）
    Wearable,
    /// IoT 设备（传感器、家电等）
    IoT,
    /// 未知
    Unknown,
}

impl OsType {
    /// 获取当前编译目标的操作系统类型（编译时常量）
    pub const fn current() -> Self {
        #[cfg(target_os = "android")]
        { OsType::Android }
        #[cfg(target_os = "ios")]
        { OsType::Ios }
        #[cfg(target_os = "ohos")]
        { OsType::Ohos }
        #[cfg(target_os = "windows")]
        { OsType::Windows }
        #[cfg(target_os = "linux")]
        { OsType::Linux }
        #[cfg(target_os = "macos")]
        { OsType::MacOS }
    }

    pub const fn is_android(&self) -> bool { matches!(self, OsType::Android) }
    pub const fn is_ios(&self) -> bool { matches!(self, OsType::Ios) }
    /// OpenHarmony / HarmonyOS
    pub const fn is_ohos(&self) -> bool { matches!(self, OsType::Ohos) }
    pub const fn is_windows(&self) -> bool { matches!(self, OsType::Windows) }
    pub const fn is_linux(&self) -> bool { matches!(self, OsType::Linux) }
    pub const fn is_macos(&self) -> bool { matches!(self, OsType::MacOS) }

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

impl DeviceForm {
    /// 获取当前设备形态（运行时检测）
    pub fn current() -> Self {
        // 需要各平台实现具体检测逻辑
        crate::platform::current::device_provider().device_form()
    }

    pub const fn is_phone(&self) -> bool { matches!(self, DeviceForm::Phone) }
    pub const fn is_tablet(&self) -> bool { matches!(self, DeviceForm::Tablet) }
    pub const fn is_desktop(&self) -> bool { matches!(self, DeviceForm::Desktop) }
    pub const fn is_tv(&self) -> bool { matches!(self, DeviceForm::Tv) }
    pub const fn is_car(&self) -> bool { matches!(self, DeviceForm::Car) }
    pub const fn is_wearable(&self) -> bool { matches!(self, DeviceForm::Wearable) }
    pub const fn is_iot(&self) -> bool { matches!(self, DeviceForm::IoT) }

    /// 是否为移动端（手机/平板）
    pub const fn is_mobile(&self) -> bool {
        matches!(self, DeviceForm::Phone | DeviceForm::Tablet)
    }
}

/// 编译时常量：当前操作系统类型
pub const CURRENT_OS: OsType = OsType::current();

/// 编译时宏：条件编译代码块
#[macro_export]
macro_rules! platform {
    (android => $block:block) => {
        #[cfg(target_os = "android")]
        $block
    };
    (ios => $block:block) => {
        #[cfg(target_os = "ios")]
        $block
    };
    (ohos => $block:block) => {
        #[cfg(target_os = "ohos")]
        $block
    };
    (windows => $block:block) => {
        #[cfg(target_os = "windows")]
        $block
    };
    (linux => $block:block) => {
        #[cfg(target_os = "linux")]
        $block
    };
    (macos => $block:block) => {
        #[cfg(target_os = "macos")]
        $block
    };
    (mobile => $block:block) => {
        #[cfg(any(target_os = "android", target_os = "ios", target_os = "ohos"))]
        $block
    };
    (desktop => $block:block) => {
        #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
        $block
    };
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

/// 屏幕方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Portrait,
    Landscape,
    Unknown,
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

/// 屏幕信息
#[derive(Debug, Clone, PartialEq)]
pub struct ScreenInfo {
    pub width: u32,
    pub height: u32,
    pub dpi: f32,
    pub scale_factor: f32,
    pub orientation: Orientation,
}
```

### error.rs

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("当前平台不支持此功能")]
    NotSupported,

    #[error("权限不足: {0}")]
    PermissionDenied(String),

    #[error("系统错误: {0}")]
    SystemError(i32),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("FFI 错误: {0}")]
    FfiError(String),
}

pub type Result<T> = std::result::Result<T, PlatformError>;
```

---

## 二、Trait 定义

### traits/device.rs — 设备信息

```rust
use crate::types::{PlatformInfo, CpuArch, OsType};
use crate::error::Result;

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
```

### traits/path.rs — 文件路径

```rust
use std::path::PathBuf;
use crate::error::Result;

/// 文件路径接口
pub trait PathProvider {
    /// 应用数据目录（持久化存储）
    ///
    /// - Android: /data/data/<pkg>/files
    /// - iOS: ~/Library/Application Support
    /// - Windows: %APPDATA%\<pkg>
    /// - Linux: ~/.local/share/<pkg>
    /// - macOS: ~/Library/Application Support/<pkg>
    fn data_dir(&self) -> Result<PathBuf>;

    /// 缓存目录（可清理）
    ///
    /// - Android: /data/data/<pkg>/cache
    /// - iOS: ~/Library/Caches
    /// - Windows: %LOCALAPPDATA%\<pkg>\Cache
    /// - Linux: ~/.cache/<pkg>
    /// - macOS: ~/Library/Caches/<pkg>
    fn cache_dir(&self) -> Result<PathBuf>;

    /// 临时目录（系统可能清理）
    fn temp_dir(&self) -> Result<PathBuf>;

    /// 文档目录（用户可见）
    ///
    /// - Android: /data/data/<pkg>/files/Documents
    /// - iOS: ~/Documents
    /// - Windows: %USERPROFILE%\Documents
    /// - Linux/MacOS: ~/Documents
    fn document_dir(&self) -> Result<PathBuf>;

    /// 应用数据目录（外部存储，持久化）
    ///
    /// - Android: /sdcard/Android/data/<pkg>/files
    /// - 其他平台: 与 data_dir() 相同
    fn external_data_dir(&self) -> Result<PathBuf>;

    /// 缓存目录（外部存储，可清理）
    ///
    /// - Android: /sdcard/Android/data/<pkg>/cache
    /// - 其他平台: 与 cache_dir() 相同
    fn external_cache_dir(&self) -> Result<PathBuf>;

    // ===== 异步接口 =====

    /// 异步获取应用数据目录
    fn data_dir_async(&self) -> Pin<Box<dyn Future<Output = Result<PathBuf>> + Send + '_>> { ... }

    /// 异步获取缓存目录
    fn cache_dir_async(&self) -> Pin<Box<dyn Future<Output = Result<PathBuf>> + Send + '_>> { ... }

    /// 异步获取临时目录
    fn temp_dir_async(&self) -> Pin<Box<dyn Future<Output = Result<PathBuf>> + Send + '_>> { ... }

    /// 异步获取文档目录
    fn document_dir_async(&self) -> Pin<Box<dyn Future<Output = Result<PathBuf>> + Send + '_>> { ... }

    /// 异步获取外部存储应用数据目录
    fn external_data_dir_async(&self) -> Pin<Box<dyn Future<Output = Result<PathBuf>> + Send + '_>> { ... }

    /// 异步获取外部存储缓存目录
    fn external_cache_dir_async(&self) -> Pin<Box<dyn Future<Output = Result<PathBuf>> + Send + '_>> { ... }
}
```

### traits/screen.rs — 屏幕信息

```rust
use crate::types::{ScreenInfo, Orientation};
use crate::error::Result;

/// 屏幕信息接口
pub trait ScreenProvider {
    /// 获取屏幕信息
    fn screen_info(&self) -> Result<ScreenInfo>;

    /// 获取屏幕宽度（像素）
    fn screen_width(&self) -> Result<u32>;

    /// 获取屏幕高度（像素）
    fn screen_height(&self) -> Result<u32>;

    /// 获取缩放因子
    fn scale_factor(&self) -> Result<f32>;

    /// 获取屏幕方向
    fn orientation(&self) -> Result<Orientation>;

    // ===== 异步接口 =====

    /// 异步获取屏幕信息
    fn screen_info_async(&self) -> Pin<Box<dyn Future<Output = Result<ScreenInfo>> + Send + '_>> { ... }

    /// 异步获取屏幕方向
    fn orientation_async(&self) -> Pin<Box<dyn Future<Output = Result<Orientation>> + Send + '_>> { ... }
}
```

---

## 三、统一接口（便捷 API）

### lib.rs

```rust
use std::path::PathBuf;
use crate::error::Result;
use crate::types::{PlatformInfo, ScreenInfo, OsType, CURRENT_OS};

// 内部使用 platform::current 获取实现
use platform::current;

// ========== 系统类型判断（编译时常量） ==========

/// 获取当前操作系统类型（编译时确定）
pub const fn current_os() -> OsType {
    CURRENT_OS
}

/// 是否为 Android
pub const fn is_android() -> bool {
    CURRENT_OS.is_android()
}

/// 是否为 iOS
pub const fn is_ios() -> bool {
    CURRENT_OS.is_ios()
}

/// 是否为 OpenHarmony / HarmonyOS
pub const fn is_ohos() -> bool {
    CURRENT_OS.is_ohos()
}

/// 是否为 Windows
pub const fn is_windows() -> bool {
    CURRENT_OS.is_windows()
}

/// 是否为 Linux
pub const fn is_linux() -> bool {
    CURRENT_OS.is_linux()
}

/// 是否为 macOS
pub const fn is_macos() -> bool {
    CURRENT_OS.is_macos()
}

// ========== 设备形态判断（运行时） ==========

/// 获取当前设备形态
pub fn current_device_form() -> DeviceForm {
    DeviceForm::current()
}

/// 是否为手机
pub fn is_phone() -> bool {
    current_device_form().is_phone()
}

/// 是否为平板
pub fn is_tablet() -> bool {
    current_device_form().is_tablet()
}

/// 是否为桌面设备
pub fn is_desktop_device() -> bool {
    current_device_form().is_desktop()
}

// ========== 平台信息 ==========

/// 获取平台信息
pub fn platform_info() -> Result<PlatformInfo> {
    current::device_provider().platform_info()
}

// ========== 路径相关 ==========

/// 获取应用数据目录
pub fn data_dir() -> Result<PathBuf> {
    current::path_provider().data_dir()
}

/// 获取缓存目录
pub fn cache_dir() -> Result<PathBuf> {
    current::path_provider().cache_dir()
}

/// 获取临时目录
pub fn temp_dir() -> Result<PathBuf> {
    current::path_provider().temp_dir()
}

/// 获取文档目录
pub fn document_dir() -> Result<PathBuf> {
    current::path_provider().document_dir()
}

// ========== 屏幕相关 ==========

/// 获取屏幕信息
pub fn screen_info() -> Result<ScreenInfo> {
    current::screen_provider().screen_info()
}
```

---

## 四、平台实现示例

### platform/android/device.rs

```rust
use crate::traits::DeviceInfo;
use crate::types::*;
use crate::error::Result;

pub struct AndroidDeviceInfo;

impl DeviceInfo for AndroidDeviceInfo {
    fn platform_info(&self) -> Result<PlatformInfo> {
        Ok(PlatformInfo {
            os_type: self.os_type(),
            os_version: self.os_version()?,
            device_model: self.device_model()?,
            cpu_arch: self.cpu_arch(),
            device_form: self.device_form(),
        })
    }

    fn os_type(&self) -> OsType {
        OsType::Android
    }

    fn os_version(&self) -> Result<String> {
        unsafe {
            let version = ffi::android_get_sdk_version();
            Ok(format!("Android API {}", version))
        }
    }

    fn device_model(&self) -> Result<String> {
        unsafe {
            let mut buf = [0u8; 256];
            let len = ffi::android_get_device_model(buf.as_mut_ptr(), buf.len());
            Ok(String::from_utf8_lossy(&buf[..len as usize]).to_string())
        }
    }

    fn cpu_arch(&self) -> CpuArch {
        #[cfg(target_arch = "aarch64")]
        { CpuArch::Arm64 }
        #[cfg(target_arch = "arm")]
        { CpuArch::Arm }
        #[cfg(target_arch = "x86_64")]
        { CpuArch::X86_64 }
        #[cfg(target_arch = "x86")]
        { CpuArch::X86 }
    }

    fn device_form(&self) -> DeviceForm {
        // 使用 NDK AConfiguration API 判断设备类型
        unsafe {
            let config = ffi::AConfiguration_new();
            ffi::AConfiguration_fromAssetManager(config, asset_manager);
            let screen_layout = ffi::AConfiguration_getScreenLayout(config);
            let size = screen_layout & ACONFIGURATION_SCREENLAYOUT_SIZE_MASK;
            ffi::AConfiguration_delete(config);

            match size {
                // SMALL/NORMAL = 手机
                ACONFIGURATION_SCREENLAYOUT_SIZE_SMALL |
                ACONFIGURATION_SCREENLAYOUT_SIZE_NORMAL => DeviceForm::Phone,
                // LARGE/XLARGE = 平板
                ACONFIGURATION_SCREENLAYOUT_SIZE_LARGE |
                ACONFIGURATION_SCREENLAYOUT_SIZE_XLARGE => DeviceForm::Tablet,
                _ => DeviceForm::Unknown,
            }
        }
    }
}
```

### platform/ios/device.rs

```rust
use crate::traits::DeviceInfo;
use crate::types::*;
use crate::error::Result;

pub struct IosDeviceInfo;

impl DeviceInfo for IosDeviceInfo {
    fn platform_info(&self) -> Result<PlatformInfo> {
        Ok(PlatformInfo {
            os_type: self.os_type(),
            os_version: self.os_version()?,
            device_model: self.device_model()?,
            cpu_arch: self.cpu_arch(),
            device_form: self.device_form(),
        })
    }

    fn os_type(&self) -> OsType {
        OsType::Ios
    }

    fn os_version(&self) -> Result<String> {
        unsafe {
            let version = ffi::UIDevice_systemVersion();
            Ok(version.to_string())
        }
    }

    fn device_model(&self) -> Result<String> {
        unsafe {
            let model = ffi::UIDevice_model();
            Ok(model.to_string())
        }
    }

    fn cpu_arch(&self) -> CpuArch {
        CpuArch::Arm64  // iOS 设备均为 ARM64
    }

    fn device_form(&self) -> DeviceForm {
        // 使用 UIDevice.userInterfaceIdiom 判断
        unsafe {
            let idiom = ffi::UIDevice_userInterfaceIdiom();
            match idiom {
                0 => DeviceForm::Phone,   // UIUserInterfaceIdiomPhone
                1 => DeviceForm::Tablet,  // UIUserInterfaceIdiomPad
                _ => DeviceForm::Unknown,
            }
        }
    }
}
```

### platform/ohos/device.rs

```rust
use crate::traits::DeviceInfo;
use crate::types::*;
use crate::error::Result;

pub struct OhosDeviceInfo;

impl DeviceInfo for OhosDeviceInfo {
    fn platform_info(&self) -> Result<PlatformInfo> {
        Ok(PlatformInfo {
            os_type: self.os_type(),
            os_version: self.os_version()?,
            device_model: self.device_model()?,
            cpu_arch: self.cpu_arch(),
            device_form: self.device_form(),
        })
    }

    fn os_type(&self) -> OsType {
        OsType::Ohos
    }

    fn os_version(&self) -> Result<String> {
        unsafe {
            let version = ffi::ohos_get_os_full_name();
            Ok(version)
        }
    }

    fn device_model(&self) -> Result<String> {
        unsafe {
            let model = ffi::ohos_get_product_model();
            Ok(model)
        }
    }

    fn cpu_arch(&self) -> CpuArch {
        #[cfg(target_arch = "aarch64")]
        { CpuArch::Arm64 }
        #[cfg(target_arch = "arm")]
        { CpuArch::Arm }
        #[cfg(target_arch = "x86_64")]
        { CpuArch::X86_64 }
        #[cfg(target_arch = "x86")]
        { CpuArch::X86 }
    }

    fn device_form(&self) -> DeviceForm {
        // 使用 deviceInfo.deviceType 判断
        // OpenHarmony 支持多种设备形态
        unsafe {
            let device_type = ffi::ohos_get_device_type();
            match device_type.as_str() {
                "phone" => DeviceForm::Phone,
                "tablet" => DeviceForm::Tablet,
                "tv" => DeviceForm::Tv,
                "wearable" | "liteWearable" => DeviceForm::Wearable,
                "2in1" => DeviceForm::Desktop,
                "car" => DeviceForm::Car,
                _ => DeviceForm::Unknown,
            }
        }
    }
}
```

### platform/windows/linux/macos/device.rs

```rust
use crate::traits::DeviceInfo;
use crate::types::*;
use crate::error::Result;

pub struct DesktopDeviceInfo;

impl DeviceInfo for DesktopDeviceInfo {
    fn platform_info(&self) -> Result<PlatformInfo> {
        Ok(PlatformInfo {
            os_type: self.os_type(),
            os_version: self.os_version()?,
            device_model: self.device_model()?,
            cpu_arch: self.cpu_arch(),
            device_form: self.device_form(),
        })
    }

    fn os_type(&self) -> OsType {
        #[cfg(target_os = "windows")]
        { OsType::Windows }
        #[cfg(target_os = "linux")]
        { OsType::Linux }
        #[cfg(target_os = "macos")]
        { OsType::MacOS }
    }

    fn os_version(&self) -> Result<String> {
        // 各平台调用各自 API
        #[cfg(target_os = "windows")]
        { /* 调用 GetVersionEx */ }
        #[cfg(target_os = "linux")]
        { /* 调用 uname */ }
        #[cfg(target_os = "macos")]
        { /* 调用 NSProcessInfo */ }
    }

    fn device_model(&self) -> Result<String> {
        #[cfg(target_os = "windows")]
        { /* 调用 GetSystemInfo */ }
        #[cfg(target_os = "linux")]
        { /* 读取 /sys/class/dmi/id/product_name */ }
        #[cfg(target_os = "macos")]
        { /* 调用 IORegistryEntry */ }
    }

    fn cpu_arch(&self) -> CpuArch {
        #[cfg(target_arch = "x86_64")]
        { CpuArch::X86_64 }
        #[cfg(target_arch = "x86")]
        { CpuArch::X86 }
        #[cfg(target_arch = "aarch64")]
        { CpuArch::Arm64 }
    }

    fn device_form(&self) -> DeviceForm {
        // 桌面系统默认为 Desktop
        DeviceForm::Desktop
    }
}
```

---

## 设备类型判断实现汇总

| 平台 | API | 判断依据 |
|------|-----|----------|
| Android | `AConfiguration_getScreenLayout` | 屏幕尺寸 SMALL/NORMAL → Phone, LARGE/XLARGE → Tablet |
| iOS | `UIDevice.userInterfaceIdiom` | 0 → Phone, 1 → Tablet |
| OpenHarmony | `deviceInfo.deviceType` | "phone"/"tablet"/"tv"/"wearable"/"2in1" |
| Windows/Linux/macOS | 固定值 | 默认 Desktop |

### platform/android/path.rs

```rust
use std::path::PathBuf;
use crate::traits::PathProvider;
use crate::error::Result;

pub struct AndroidPathProvider;

impl PathProvider for AndroidPathProvider {
    fn data_dir(&self) -> Result<PathBuf> {
        // 调用 Android NDK API
        unsafe {
            let mut buf = [0u8; 512];
            let len = ffi::android_get_data_dir(buf.as_mut_ptr(), buf.len());
            let path = String::from_utf8_lossy(&buf[..len as usize]);
            Ok(PathBuf::from(path.to_string()))
        }
    }

    fn cache_dir(&self) -> Result<PathBuf> {
        unsafe {
            let mut buf = [0u8; 512];
            let len = ffi::android_get_cache_dir(buf.as_mut_ptr(), buf.len());
            let path = String::from_utf8_lossy(&buf[..len as usize]);
            Ok(PathBuf::from(path.to_string()))
        }
    }

    fn temp_dir(&self) -> Result<PathBuf> {
        Ok(PathBuf::from("/tmp"))
    }

    fn document_dir(&self) -> Result<PathBuf> {
        Ok(self.data_dir()?.join("Documents"))
    }
}
```

---

## 五、使用示例

### 业务代码

```rust
use rust_platform_adapter::prelude::*;

fn main() -> Result<()> {
    // 获取平台信息
    let info = platform_info()?;
    println!("OS: {} {}", info.os_type, info.os_version);
    println!("Device: {}", info.device_model);

    // 获取路径
    let db_path = data_dir()?.join("app.db");
    let cache = cache_dir()?;
    println!("DB path: {:?}", db_path);
    println!("Cache: {:?}", cache);

    // 获取屏幕信息
    let screen = screen_info()?;
    println!("Screen: {}x{}", screen.width, screen.height);

    Ok(())
}
```

---

## 六、v1.0 接口清单

### 系统类型判断（编译时常量，零开销）

| 接口 | 说明 |
|------|------|
| `current_os()` | 获取当前操作系统类型 |
| `is_android()` | 是否为 Android |
| `is_ios()` | 是否为 iOS |
| `is_ohos()` | 是否为 OpenHarmony / HarmonyOS |
| `is_windows()` | 是否为 Windows |
| `is_linux()` | 是否为 Linux |
| `is_macos()` | 是否为 macOS |
| `CURRENT_OS` | 编译时常量 |
| `platform!()` | 条件编译宏 |

### 设备形态判断（运行时）

| 接口 | 说明 |
|------|------|
| `current_device_form()` | 获取当前设备形态 |
| `is_phone()` | 是否为手机 |
| `is_tablet()` | 是否为平板 |
| `is_desktop_device()` | 是否为桌面设备 |

> **说明**：OpenHarmony 支持多种设备形态（手机/平板/PC/IoT），不能简单归类为 mobile。设备形态需要运行时检测。

### 设备信息

| 接口 | 说明 |
|------|------|
| `platform_info()` | 获取完整平台信息 |
| `os_version()` | 获取操作系统版本 |
| `device_model()` | 获取设备型号 |
| `cpu_arch()` | 获取 CPU 架构 |
| `device_form()` | 获取设备形态 |

### 文件路径

| 接口 | 说明 |
|------|------|
| `data_dir()` | 应用数据目录 |
| `cache_dir()` | 缓存目录 |
| `temp_dir()` | 临时目录 |
| `document_dir()` | 文档目录 |

### 屏幕信息

| 接口 | 说明 |
|------|------|
| `screen_info()` | 获取完整屏幕信息 |
| `screen_width()` | 屏幕宽度 |
| `screen_height()` | 屏幕高度 |
| `scale_factor()` | 缩放因子 |
| `orientation()` | 屏幕方向 |
