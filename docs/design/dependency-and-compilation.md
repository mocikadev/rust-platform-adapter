# 依赖方式与跨平台编译机制

## 一、依赖方式

业务模块可通过三种方式依赖本库，按场景选择：

| 方式 | 适用场景 | 稳定性 |
|------|----------|--------|
| 源码依赖 | 开发期、本地调试 | 随源码变动 |
| Git 依赖 | 未发布、团队内部使用 | 指定 branch/tag |
| crates.io | 正式发布 | 语义化版本 |

### 1.1 源码依赖（开发期）

```toml
[dependencies]
rust-platform-adapter = { path = "../rust-platform-adapter" }
```

### 1.2 Git 依赖（团队内部）

```toml
[dependencies]
# 指定分支
rust-platform-adapter = { git = "https://github.com/your-org/rust-platform-adapter", branch = "main" }

# 指定 tag（推荐，版本明确）
rust-platform-adapter = { git = "https://github.com/your-org/rust-platform-adapter", tag = "v0.1.0" }

# 指定 commit
rust-platform-adapter = { git = "https://github.com/your-org/rust-platform-adapter", rev = "abc1234" }
```

### 1.3 crates.io 依赖（正式发布）

```toml
[dependencies]
rust-platform-adapter = "0.1.0"
```

### 1.4 依赖方式切换

开发阶段建议使用源码依赖，发布前切换为 Git 或 crates.io：

```toml
# 本地开发
[dependencies]
rust-platform-adapter = { path = "../rust-platform-adapter" }

# 发布时切换为
[dependencies]
rust-platform-adapter = { git = "https://github.com/your-org/rust-platform-adapter", tag = "v0.1.0" }
```

---

## 二、跨平台编译机制

### 2.1 核心原理

Rust 使用 `#[cfg(target_os)]` 条件编译，根据编译目标自动选择平台实现：

```
cargo build --target <TARGET>
                │
                ▼
        #[cfg(target_os = "xxx")]
                │
                ▼
        只编译对应平台代码
```

### 2.2 条件编译示例

```rust
// src/platform/mod.rs

// 编译时根据 target 自动选择，无需运行时判断
#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

// 导出当前平台实现
#[cfg(target_os = "android")]
pub use android::*;

#[cfg(target_os = "ios")]
pub use ios::*;

#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "macos")]
pub use macos::*;
```

### 2.3 编译目标清单

| 平台 | target triple | 说明 |
|------|---------------|------|
| Android (ARM64) | `aarch64-linux-android` | 主流 Android 设备 |
| Android (ARMv7) | `armv7-linux-androideabi` | 旧设备 |
| Android (x86_64) | `x86_64-linux-android` | 模拟器 |
| iOS (ARM64) | `aarch64-apple-ios` | 真机 |
| iOS (x86_64) | `x86_64-apple-ios` | 模拟器 |
| Windows (x64) | `x86_64-pc-windows-msvc` | 主流 Windows |
| Linux (x64) | `x86_64-unknown-linux-gnu` | 主流 Linux |
| Linux (ARM64) | `aarch64-unknown-linux-gnu` | 树莓派等 |
| macOS (x64) | `x86_64-apple-darwin` | Intel Mac |
| macOS (ARM64) | `aarch64-apple-darwin` | Apple Silicon |

### 2.4 编译命令

```bash
# Android
cargo build --target aarch64-linux-android

# iOS
cargo build --target aarch64-apple-ios

# Windows
cargo build --target x86_64-pc-windows-msvc

# Linux
cargo build --target x86_64-unknown-linux-gnu

# macOS
cargo build --target x86_64-apple-darwin
```

### 2.5 交叉编译环境准备

```bash
# 安装目标平台工具链
rustup target add aarch64-linux-android
rustup target add aarch64-apple-ios
rustup target add x86_64-pc-windows-msvc

# Android NDK（需单独安装）
export ANDROID_NDK_HOME=/path/to/ndk

# 验证安装
rustup target list --installed
```

---

## 三、业务模块使用示例

### 3.1 业务模块 Cargo.toml

```toml
[package]
name = "my-app"
version = "0.1.0"

[dependencies]
rust-platform-adapter = { path = "../rust-platform-adapter" }
```

### 3.2 业务代码

```rust
use rust_platform_adapter::prelude::*;

fn main() -> Result<()> {
    // 获取平台信息
    let info = platform_info()?;
    println!("OS: {} {}", info.os_type(), info.os_version());

    // 获取路径
    let data = data_dir()?;
    let db_path = data.join("app.db");

    Ok(())
}
```

### 3.3 编译业务模块

```bash
# 编译 Android 版本
cargo build --target aarch64-linux-android

# 编译 iOS 版本
cargo build --target aarch64-apple-ios
```

业务模块无需任何平台相关配置，只需指定 `--target`，库内部自动选择实现。

---

## 四、编译流程图

```
业务模块 Cargo.toml
       │
       │  依赖 rust-platform-adapter
       ▼
cargo build --target aarch64-linux-android
       │
       ▼
rust-platform-adapter 编译
       │
       │  遇到 #[cfg(target_os = "android")]
       │  其他平台代码被排除
       ▼
只编译 android/ 目录实现
       │
       ▼
链接到业务模块
       │
       ▼
生成 Android 平台二进制
```
