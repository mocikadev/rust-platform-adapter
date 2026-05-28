# rust-platform-adapter

Rust 跨平台适配层，统一接口调用各平台 Native API，实现零开销抽象。

## 支持平台

| 平台 | 系统标识 | 实现方式 |
|------|----------|----------|
| Android | `target_os = "android"` | NDK (ndk crate) |
| iOS | `target_os = "ios"` | objc2 |
| OpenHarmony | `target_os = "ohos"` | NDK C API |
| Windows | `target_os = "windows"` | dirs + sysinfo |
| Linux | `target_os = "linux"` | dirs + sysinfo |
| macOS | `target_os = "macos"` | dirs + sysinfo |

## 快速开始

### 添加依赖

```toml
[dependencies]
rust-platform-adapter = { git = "https://github.com/your-org/rust-platform-adapter" }
```

### 使用示例

```rust
use rust_platform_adapter::prelude::*;

fn main() -> Result<()> {
    // 系统类型判断（编译时常量，零开销）
    if is_android() {
        println!("Running on Android");
    }
    if is_ohos() {
        println!("Running on OpenHarmony/HarmonyOS");
    }

    // 获取平台信息
    let info = platform_info()?;
    println!("OS: {:?}", info.os_type);
    println!("Version: {}", info.os_version);
    println!("Model: {}", info.device_model);
    println!("Arch: {:?}", info.cpu_arch);
    println!("Form: {:?}", info.device_form);

    // 获取路径
    println!("Data: {:?}", data_dir()?);
    println!("Cache: {:?}", cache_dir()?);
    println!("Temp: {:?}", temp_dir()?);
    println!("Documents: {:?}", document_dir()?);

    // 获取屏幕信息
    let screen = screen_info()?;
    println!("Screen: {}x{}", screen.width, screen.height);
    println!("DPI: {}", screen.dpi);
    println!("Scale: {}", screen.scale_factor);

    // 设备形态判断
    if device_form().is_mobile() {
        println!("Running on mobile device");
    }
    if device_form().is_desktop() {
        println!("Running on desktop");
    }

    Ok(())
}
```

## API 清单

### 系统类型判断（编译时常量）

| 函数 | 说明 |
|------|------|
| `current_os()` | 获取当前操作系统类型 |
| `is_android()` | 是否为 Android |
| `is_ios()` | 是否为 iOS |
| `is_ohos()` | 是否为 OpenHarmony/HarmonyOS |
| `is_windows()` | 是否为 Windows |
| `is_linux()` | 是否为 Linux |
| `is_macos()` | 是否为 macOS |

### 平台信息

| 函数 | 说明 |
|------|------|
| `platform_info()` | 获取完整平台信息 |
| `os_version()` | 获取操作系统版本 |
| `device_model()` | 获取设备型号 |
| `cpu_arch()` | 获取 CPU 架构 |
| `device_form()` | 获取设备形态 |

### 文件路径

| 函数 | 说明 |
|------|------|
| `data_dir()` | 应用数据目录 |
| `cache_dir()` | 缓存目录 |
| `temp_dir()` | 临时目录 |
| `document_dir()` | 文档目录 |

### 屏幕信息

| 函数 | 说明 |
|------|------|
| `screen_info()` | 获取完整屏幕信息 |
| `screen_width()` | 屏幕宽度 |
| `screen_height()` | 屏幕高度 |
| `scale_factor()` | 缩放因子 |
| `orientation()` | 屏幕方向 |

## 编译

### 当前平台

```bash
cargo build
cargo test
```

### 交叉编译

```bash
# Android
cargo build --target aarch64-linux-android
cargo build --target armv7-linux-androideabi
cargo build --target x86_64-linux-android

# iOS
cargo build --target aarch64-apple-ios

# OpenHarmony
cargo build --target aarch64-unknown-linux-ohos
cargo build --target x86_64-unknown-linux-ohos

# Windows
cargo build --target x86_64-pc-windows-gnu
```

## 项目结构

```
rust-platform-adapter/
├── Cargo.toml              # workspace 根配置
├── .cargo/config.toml      # 交叉编译 linker 配置
├── crates/
│   ├── adapter/            # 核心适配库
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs      # 入口 + prelude
│   │       ├── api.rs      # 便捷 API
│   │       ├── types.rs    # 公共类型（OsType, DeviceForm, CpuArch 等）
│   │       ├── error.rs    # 错误类型（PlatformError）
│   │       ├── traits/     # Trait 定义
│   │       │   ├── device.rs
│   │       │   ├── path.rs
│   │       │   └── screen.rs
│   │       └── platform/   # 平台实现
│   │           ├── linux/
│   │           ├── windows/
│   │           ├── macos/
│   │           ├── ios/
│   │           ├── android/
│   │           └── ohos/
│   └── test-app/           # 测试应用
└── docs/                   # 文档
```

## 依赖

### 公共依赖

- `thiserror` — 错误处理
- `dirs` — 标准目录路径（桌面平台）
- `sysinfo` — 系统信息（桌面平台）

### 平台特定依赖

- Android: `ndk`
- iOS/macOS: `objc2`, `objc2-ui-kit`, `objc2-app-kit`, `objc2-foundation`
- OpenHarmony: NDK C API（`libdeviceinfo_ndk.z.so`, `libability_runtime.so`, `libnative_display_manager.so`）

## 许可证

MIT
