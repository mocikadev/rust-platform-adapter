# 平台 API 调研报告

## 一、调研目标

确认各目标平台是否提供 C/C++ 接口，以及现有 Rust 生态的覆盖情况。

## 二、目标平台

| 平台 | 系统标识 | 桥接方式 |
|------|----------|----------|
| Android | `target_os = "android"` | NDK (C) |
| iOS | `target_os = "ios"` | ObjC/C |
| OpenHarmony | `target_os = "ohos"` | NAPI (C) |
| Windows | `target_os = "windows"` | Win32 (C) |
| Linux | `target_os = "linux"` | POSIX (C) |
| macOS | `target_os = "macos"` | ObjC/C |

## 三、各平台 C/C++ 接口情况

### 3.1 Android — NDK

| 能力 | API | 说明 |
|------|-----|------|
| 设备信息 | `AConfiguration` | 屏幕尺寸、语言、密度等 |
| API 版本 | `android_get_device_api_level()` | API level |
| 文件路径 | `AAssetManager` | 应用资源路径 |
| 系统属性 | `__system_property_get()` | 设备型号、品牌等 |

#### Android Rust 生态 — ndk / jni

**相关 Crates**：

| Crate | 最新版本 | 最近更新 | 说明 |
|-------|----------|----------|------|
| `ndk` | 0.9.0 | 2024-04-26 | NDK 原生 API 绑定 |
| `jni` | 0.22.4 | 2026-03-16 | JNI 绑定 |

**仓库**：
- ndk: https://github.com/rust-mobile/ndk
- jni: https://github.com/jni-rs/jni-rs

**依赖配置**：
```toml
[target.'cfg(target_os = "android")'.dependencies]
ndk = "0.9.0"
jni = "0.22.4"
```

**使用示例**：
```rust
use ndk::configuration::AConfiguration;

fn get_device_form() -> DeviceForm {
    let config = AConfiguration::new();
    let screen_layout = config.screen_layout();
    let size = screen_layout & ACONFIGURATION_SCREENLAYOUT_SIZE_MASK;
    
    match size {
        ACONFIGURATION_SCREENLAYOUT_SIZE_SMALL |
        ACONFIGURATION_SCREENLAYOUT_SIZE_NORMAL => DeviceForm::Phone,
        ACONFIGURATION_SCREENLAYOUT_SIZE_LARGE |
        ACONFIGURATION_SCREENLAYOUT_SIZE_XLARGE => DeviceForm::Tablet,
        _ => DeviceForm::Unknown,
    }
}
```

> **结论**：Android 有成熟的 Rust 绑定库 `ndk` 和 `jni`，可直接复用

### 3.2 iOS — UIKit/Foundation (ObjC)

| 能力 | API | 说明 |
|------|-----|------|
| 设备信息 | `UIDevice` | 设备型号、系统版本、设备类型 |
| 设备类型 | `userInterfaceIdiom` | iPhone / iPad |
| 文件路径 | `NSSearchPathForDirectoriesInDomains` | Documents、Caches 等 |
| 屏幕信息 | `UIScreen` | 分辨率、缩放因子 |

#### iOS Rust 生态 — objc2

**项目地址**：https://github.com/madsmtm/objc2

**相关 Crates**：

| Crate | 最新版本 | 最近更新 | 说明 |
|-------|----------|----------|------|
| `objc2` | 0.6.4 | 2026-02-26 | ObjC 核心绑定 |
| `objc2-foundation` | 0.3.2 | 2025-10-04 | Foundation 框架 |
| `objc2-ui-kit` | 0.3.2 | 2025-10-04 | UIKit 框架 (iOS) |

**依赖配置**：
```toml
[target.'cfg(target_os = "ios")'.dependencies]
objc2 = "0.6.4"
objc2-ui-kit = "0.3.2"
objc2-foundation = "0.3.2"
```

**使用示例**：
```rust
use objc2_ui_kit::UIDevice;

fn get_device_form() -> DeviceForm {
    let device = unsafe { UIDevice::currentDevice() };
    let idiom = unsafe { device.userInterfaceIdiom() };
    
    match idiom {
        0 => DeviceForm::Phone,
        1 => DeviceForm::Tablet,
        _ => DeviceForm::Unknown,
    }
}
```

> **结论**：iOS 有成熟的 Rust 绑定库 `objc2`，活跃维护，可直接复用

### 3.3 OpenHarmony — NAPI

| 能力 | API | 说明 |
|------|-----|------|
| 设备信息 | `@ohos.deviceInfo` | deviceType、brand、model 等 |
| 设备类型 | `deviceInfo.deviceType` | phone/tablet/tv/wearable/2in1 |
| 文件路径 | `@ohos.file.environment` | 数据目录、缓存目录 |
| 屏幕信息 | `@ohos.display` | 分辨率、DPI |

> OpenHarmony 的 NAPI 提供 C 接口，可通过 `ohos-rs` 框架调用

#### OpenHarmony Rust 生态 — ohos-rs

**项目地址**：https://github.com/ohos-rs/ohos-rs  
**文档**：https://ohos.rs

**相关 Crates**（均在 crates.io 上，持续活跃维护）：

| Crate | 最新版本 | 最近更新 | 说明 |
|-------|----------|----------|------|
| `napi-ohos` | 1.2.0 | 2026-05-12 | NAPI 核心绑定 |
| `napi-sys-ohos` | 1.2.0 | 2026-05-12 | NAPI 系统级绑定 |
| `napi-derive-ohos` | 1.2.0 | 2026-05-12 | 过程宏支持 |

**版本发布历史**：
```
1.2.0   2026-05-12  (最新)
1.1.6   2026-01-19
1.1.5   2025-12-29
1.1.4   2025-11-05
1.1.3   2025-10-04
```

**功能特性**：
- 从 napi-rs 分叉，专门适配 OpenHarmony
- 支持同步/异步函数
- 支持回调函数
- 自动模块注册
- 支持 arm64/arm/x86_64 架构

**依赖配置**：
```toml
[dependencies]
napi-ohos = "1.2.0"
napi-derive-ohos = "1.2.0"
```

**使用示例**：
```rust
use napi_ohos::bindgen_prelude::*;
use napi_derive_ohos::napi;

#[napi]
pub fn get_device_type() -> String {
    // 调用 OpenHarmony deviceInfo.deviceType
    "phone".to_string()
}
```

> **结论**：OpenHarmony 有成熟的 Rust 绑定库 `ohos-rs`，持续活跃维护，可直接复用

### 3.4 Windows — Win32 API

| 能力 | API | 说明 |
|------|-----|------|
| 设备信息 | `GetSystemInfo` | CPU 架构、处理器数量 |
| 系统版本 | `GetVersionExW` | Windows 版本号 |
| 文件路径 | `SHGetFolderPathW` | AppData、LocalAppData 等 |
| 屏幕信息 | `GetSystemMetrics` | 分辨率 |

### 3.5 Linux — POSIX

| 能力 | API | 说明 |
|------|-----|------|
| 设备信息 | `uname()` | 内核版本、主机名、架构 |
| 文件路径 | XDG 规范 | `~/.local/share`、`~/.cache` 等 |
| 屏幕信息 | X11/Wayland | 需额外依赖 |

### 3.6 macOS — Foundation (ObjC)

| 能力 | API | 说明 |
|------|-----|------|
| 设备信息 | `NSProcessInfo` | 系统版本、主机名、CPU 数量 |
| 文件路径 | `NSSearchPathForDirectoriesInDomains` | Application Support、Caches |
| 屏幕信息 | `NSScreen` | 分辨率、缩放因子 |

#### macOS Rust 生态 — objc2

**相关 Crates**：

| Crate | 最新版本 | 最近更新 | 说明 |
|-------|----------|----------|------|
| `objc2` | 0.6.4 | 2026-02-26 | ObjC 核心绑定 |
| `objc2-foundation` | 0.3.2 | 2025-10-04 | Foundation 框架 |
| `objc2-app-kit` | 0.3.2 | 2025-10-04 | AppKit 框架 (macOS) |

**依赖配置**：
```toml
[target.'cfg(target_os = "macos")'.dependencies]
objc2 = "0.6.4"
objc2-app-kit = "0.3.2"
objc2-foundation = "0.3.2"
```

**使用示例**：
```rust
use objc2_foundation::NSProcessInfo;

fn get_os_version() -> String {
    let info = unsafe { NSProcessInfo::processInfo() };
    let version = unsafe { info.operatingSystemVersionString() };
    version.to_string()
}
```

> **结论**：macOS 与 iOS 共用 `objc2` 绑定库，活跃维护，可直接复用

---

## 四、现有 Rust 库

### 4.1 sysinfo

- **功能**：系统信息（CPU、内存、磁盘、网络、进程）
- **平台支持**：Android ✅、iOS ✅、Windows ✅、Linux ✅、macOS ✅、OpenHarmony ❌
- **仓库**：https://github.com/GuillaumeGomez/sysinfo

### 4.2 dirs / directories

- **功能**：标准用户目录路径
- **平台支持**：Windows ✅、Linux ✅、macOS ✅、Android ❌、iOS ❌、OpenHarmony ❌
- **仓库**：https://github.com/dirs-dev/directories-rs

### 4.3 覆盖情况汇总

| 库 | Android | iOS | OpenHarmony | Windows | Linux | macOS |
|---|---------|-----|-------------|---------|-------|-------|
| `sysinfo` | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ |
| `dirs` | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ |
| `ndk` / `jni` | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| `objc2` | ❌ | ✅ | ❌ | ❌ | ❌ | ✅ |
| `ohos-rs` | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ |

**结论**：**所有平台均有成熟 Rust 绑定库**，无需手写 FFI。

---

## 五、实现策略

### 混合方案：复用现有库 + FFI 适配

```rust
// 桌面系统：复用现有 Rust 库
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
{
    // 使用 dirs 库获取路径
    // 使用 sysinfo 库获取系统信息
}

// 移动端/OpenHarmony：FFI 调用平台 API
#[cfg(any(target_os = "android", target_os = "ios", target_os = "ohos"))]
{
    // FFI 调用各平台 C/C++ API
}
```

### 优点

1. **桌面系统**：复用成熟库，减少维护成本，社区支持
2. **移动端/OpenHarmony**：FFI 调用，覆盖所有平台
3. **统一接口**：业务层无感知，透明切换

### 维护成本

| 平台 | 实现方式 | 维护成本 |
|------|----------|----------|
| Windows/Linux/macOS | 复用 `dirs` + `sysinfo` | 低 |
| Android | 复用 `ndk` | 低 |
| iOS | 复用 `objc2` | 低 |
| macOS | 复用 `objc2` | 低 |
| OpenHarmony | 复用 `ohos-rs` | 低 |

---

## 六、结论

1. **所有平台都有 C/C++ 接口**，可通过 FFI 调用
2. **所有平台均有成熟 Rust 绑定库**：
   - 桌面：`dirs` + `sysinfo`
   - Android：`ndk` + `jni`
   - iOS/macOS：`objc2`
   - OpenHarmony：`ohos-rs`
3. **推荐方案**：复用现有 Rust 绑定库，维护成本低
4. **无需手写 FFI**，所有平台都有现成绑定
