# 开发任务清单

## 阶段一：项目骨架

| 任务 | 说明 | 状态 |
|------|------|------|
| 1.1 创建 workspace | `Cargo.toml` 配置 workspace 成员 | 待开始 |
| 1.2 创建目录结构 | `crates/adapter/` 和 `crates/test-app/` | 待开始 |
| 1.3 配置依赖 | 各平台 Rust 绑定库 | 待开始 |
| 1.4 配置交叉编译 | `.cargo/config.toml` | 待开始 |

### 1.1 workspace Cargo.toml

```toml
[workspace]
members = [
    "crates/adapter",
    "crates/test-app",
]
resolver = "2"
```

### 1.3 各平台依赖

```toml
# crates/adapter/Cargo.toml

[dependencies]
thiserror = "2"
dirs = "6"
sysinfo = "0.34"

# Android
[target.'cfg(target_os = "android")'.dependencies]
ndk = "0.9.0"

# iOS
[target.'cfg(target_os = "ios")'.dependencies]
objc2 = "0.6.4"
objc2-ui-kit = "0.3.2"
objc2-foundation = "0.3.2"

# macOS
[target.'cfg(target_os = "macos")'.dependencies]
objc2 = "0.6.4"
objc2-app-kit = "0.3.2"
objc2-foundation = "0.3.2"

# OpenHarmony
[target.'cfg(target_os = "ohos")'.dependencies]
napi-ohos = "1.2.0"
napi-derive-ohos = "1.2.0"
```

---

## 阶段二：核心接口

| 任务 | 说明 | 状态 |
|------|------|------|
| 2.1 公共类型 | `OsType`, `DeviceForm`, `CpuArch`, `PlatformInfo`, `ScreenInfo` | 待开始 |
| 2.2 错误类型 | `PlatformError` 枚举 | 待开始 |
| 2.3 Trait 定义 | `DeviceInfo`, `PathProvider`, `ScreenProvider` | 待开始 |
| 2.4 便捷 API | `platform_info()`, `data_dir()`, `is_android()` 等 | 待开始 |

### 2.1 公共类型

```rust
// types.rs

pub enum OsType {
    Android,
    Ios,
    Ohos,
    Windows,
    Linux,
    MacOS,
}

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

pub enum CpuArch {
    X86,
    X86_64,
    Arm,
    Arm64,
    Unknown,
}

pub struct PlatformInfo {
    pub os_type: OsType,
    pub os_version: String,
    pub device_model: String,
    pub cpu_arch: CpuArch,
    pub device_form: DeviceForm,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScreenInfo {
    pub width: u32,
    pub height: u32,
    pub dpi: f32,
    pub scale_factor: f32,
    pub orientation: Orientation,
}
```

### 2.2 错误类型

```rust
// error.rs

#[derive(Debug, thiserror::Error)]
pub enum PlatformError {
    #[error("当前平台不支持此功能")]
    NotSupported,
    #[error("权限不足: {0}")]
    PermissionDenied(String),
    #[error("系统错误: {0}")]
    SystemError(i32),
    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, PlatformError>;
```

### 2.3 Trait 定义

```rust
// traits/device.rs
pub trait DeviceInfo {
    fn platform_info(&self) -> Result<PlatformInfo>;
    fn os_type(&self) -> OsType;
    fn os_version(&self) -> Result<String>;
    fn device_model(&self) -> Result<String>;
    fn cpu_arch(&self) -> CpuArch;
    fn device_form(&self) -> DeviceForm;
}

// traits/path.rs
pub trait PathProvider {
    fn data_dir(&self) -> Result<PathBuf>;
    fn cache_dir(&self) -> Result<PathBuf>;
    fn temp_dir(&self) -> Result<PathBuf>;
    fn document_dir(&self) -> Result<PathBuf>;
}

// traits/screen.rs
pub trait ScreenProvider {
    fn screen_info(&self) -> Result<ScreenInfo>;
    fn screen_width(&self) -> Result<u32>;
    fn screen_height(&self) -> Result<u32>;
    fn scale_factor(&self) -> Result<f32>;
}
```

---

## 阶段三：平台实现

| 平台 | 绑定库 | DeviceInfo | PathProvider | ScreenProvider | 状态 |
|------|--------|------------|--------------|----------------|------|
| 3.1 Android | `ndk` | ✅ | ✅ | ✅ | 待开始 |
| 3.2 iOS | `objc2` | ✅ | ✅ | ✅ | 待开始 |
| 3.3 macOS | `objc2` | ✅ | ✅ | ✅ | 待开始 |
| 3.4 OpenHarmony | `ohos-rs` | ✅ | ✅ | ✅ | 待开始 |
| 3.5 Windows | `dirs` + `sysinfo` | ✅ | ✅ | ✅ | 待开始 |
| 3.6 Linux | `dirs` + `sysinfo` | ✅ | ✅ | ✅ | 待开始 |

### 各平台实现说明

#### Android

```rust
// platform/android/device.rs
use ndk::configuration::AConfiguration;

impl DeviceInfo for AndroidDeviceInfo {
    fn device_form(&self) -> DeviceForm {
        let config = AConfiguration::new();
        let screen_layout = config.screen_layout();
        let size = screen_layout & ACONFIGURATION_SCREENLAYOUT_SIZE_MASK;
        match size {
            SIZE_SMALL | SIZE_NORMAL => DeviceForm::Phone,
            SIZE_LARGE | SIZE_XLARGE => DeviceForm::Tablet,
            _ => DeviceForm::Unknown,
        }
    }
}

// platform/android/screen.rs
use ndk::native_window::ANativeWindow;

impl ScreenProvider for AndroidScreenProvider {
    fn screen_info(&self) -> Result<ScreenInfo> {
        // 通过 ANativeWindow 或 JNI 获取屏幕信息
    }
}
```

#### iOS

```rust
// platform/ios/device.rs
use objc2_ui_kit::UIDevice;

impl DeviceInfo for IosDeviceInfo {
    fn device_form(&self) -> DeviceForm {
        let device = unsafe { UIDevice::currentDevice() };
        let idiom = unsafe { device.userInterfaceIdiom() };
        match idiom {
            0 => DeviceForm::Phone,
            1 => DeviceForm::Tablet,
            _ => DeviceForm::Unknown,
        }
    }
}

// platform/ios/screen.rs
use objc2_ui_kit::UIScreen;

impl ScreenProvider for IosScreenProvider {
    fn screen_info(&self) -> Result<ScreenInfo> {
        let screen = unsafe { UIScreen::mainScreen() };
        let bounds = unsafe { screen.bounds() };
        let scale = unsafe { screen.scale() };
        Ok(ScreenInfo {
            width: bounds.size.width as u32,
            height: bounds.size.height as u32,
            dpi: (scale * 163.0) as f32,  // iPhone 基准 DPI
            scale_factor: scale as f32,
        })
    }
}
```

#### macOS

```rust
// platform/macos/device.rs
use objc2_foundation::NSProcessInfo;

impl DeviceInfo for MacosDeviceInfo {
    fn os_version(&self) -> Result<String> {
        let info = unsafe { NSProcessInfo::processInfo() };
        let version = unsafe { info.operatingSystemVersionString() };
        Ok(version.to_string())
    }
}

// platform/macos/screen.rs
use objc2_app_kit::NSScreen;

impl ScreenProvider for MacosScreenProvider {
    fn screen_info(&self) -> Result<ScreenInfo> {
        let screen = unsafe { NSScreen::mainScreen() };
        let frame = unsafe { screen.frame() };
        let scale = unsafe { screen.backingScaleFactor() };
        Ok(ScreenInfo {
            width: frame.size.width as u32,
            height: frame.size.height as u32,
            dpi: 72.0 * scale as f32,  // macOS 基准 72 DPI
            scale_factor: scale as f32,
        })
    }
}
```

#### OpenHarmony

```rust
// platform/ohos/device.rs
impl DeviceInfo for OhosDeviceInfo {
    fn device_form(&self) -> DeviceForm {
        // 通过 NAPI 调用 deviceInfo.deviceType
        let device_type = ffi::ohos_get_device_type();
        match device_type.as_str() {
            "phone" => DeviceForm::Phone,
            "tablet" => DeviceForm::Tablet,
            "tv" => DeviceForm::Tv,
            "wearable" => DeviceForm::Wearable,
            "2in1" => DeviceForm::Desktop,
            _ => DeviceForm::Unknown,
        }
    }
}
```

#### Windows

```rust
// platform/windows/device.rs
use sysinfo::System;

impl DeviceInfo for WindowsDeviceInfo {
    fn os_version(&self) -> Result<String> {
        Ok(System::os_version().unwrap_or_default())
    }
}

// platform/windows/screen.rs
use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

impl ScreenProvider for WindowsScreenProvider {
    fn screen_info(&self) -> Result<ScreenInfo> {
        unsafe {
            let width = GetSystemMetrics(SM_CXSCREEN);
            let height = GetSystemMetrics(SM_CYSCREEN);
            Ok(ScreenInfo {
                width: width as u32,
                height: height as u32,
                dpi: 96.0,  // Windows 默认 DPI
                scale_factor: 1.0,
            })
        }
    }
}
```

#### Linux

```rust
// platform/linux/device.rs
use sysinfo::System;

impl DeviceInfo for LinuxDeviceInfo {
    fn os_version(&self) -> Result<String> {
        Ok(System::os_version().unwrap_or_default())
    }
}

// platform/linux/screen.rs
// 可通过 X11/Xlib 或读取 /sys/class/drm 获取
impl ScreenProvider for LinuxScreenProvider {
    fn screen_info(&self) -> Result<ScreenInfo> {
        // 方案1: 调用 X11 API (需要 x11 crate)
        // 方案2: 读取 /sys/class/drm/card0/card0-HDMI-A-1/modes
        // 方案3: 调用 xrandr 命令解析输出
        // v1.0 可先返回默认值，后续完善
        Err(PlatformError::NotSupported)
    }
}
```

---

## 阶段四：测试

| 任务 | 说明 | 状态 |
|------|------|------|
| 4.1 单元测试 | 各模块测试 | 待开始 |
| 4.2 集成测试 | 跨平台功能验证 | 待开始 |
| 4.3 test-app | 测试应用验证 | 待开始 |

### 4.1 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_type() {
        let os = OsType::current();
        #[cfg(target_os = "linux")]
        assert_eq!(os, OsType::Linux);
    }

    #[test]
    fn test_device_form() {
        let form = DeviceForm::current();
        // 无法断言具体值，只验证不 panic
    }
}
```

### 4.3 test-app

```rust
// crates/test-app/src/main.rs
use rust_platform_adapter::prelude::*;

fn main() -> Result<()> {
    println!("=== 平台信息 ===");
    println!("OS: {:?}", current_os());
    println!("设备形态: {:?}", current_device_form());
    
    let info = platform_info()?;
    println!("系统版本: {}", info.os_version);
    println!("设备型号: {}", info.device_model);
    println!("CPU 架构: {:?}", info.cpu_arch);
    
    println!("\n=== 路径信息 ===");
    println!("数据目录: {:?}", data_dir()?);
    println!("缓存目录: {:?}", cache_dir()?);
    println!("临时目录: {:?}", temp_dir()?);
    println!("文档目录: {:?}", document_dir()?);
    
    println!("\n=== 屏幕信息 ===");
    let screen = screen_info()?;
    println!("分辨率: {}x{}", screen.width, screen.height);
    println!("DPI: {}", screen.dpi);
    println!("缩放因子: {}", screen.scale_factor);
    
    Ok(())
}
```

---

## 阶段五：文档与示例

| 任务 | 说明 | 状态 |
|------|------|------|
| 5.1 API 文档 | `cargo doc` 生成 | 待开始 |
| 5.2 使用示例 | `examples/` 目录 | 待开始 |
| 5.3 README | 项目说明 | 待开始 |

---

## 开发顺序建议

```
阶段一 (项目骨架)
    ↓
阶段二 (核心接口)
    ↓
阶段三 (平台实现) → 先实现 Linux (当前环境)，验证架构
    ↓
阶段四 (测试)
    ↓
阶段五 (文档)
```

## 优先级

1. **Linux** — 当前开发环境，可立即测试
2. **Windows** — 与 Linux 共用 `dirs` + `sysinfo`
3. **macOS** — 与 iOS 共用 `objc2`
4. **iOS** — 需 macOS 编译
5. **Android** — 需 NDK 环境
6. **OpenHarmony** — 需 OpenHarmony SDK
