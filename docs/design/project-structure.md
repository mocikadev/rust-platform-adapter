# 工程结构设计

## 选定方案：Workspace 多 crate

基于以下考量选择此方案：
- **隔离性**：测试应用与核心适配库完全分离，互不干扰
- **扩展性**：新增平台特性只需在 `traits/` 和各平台目录各加一个文件
- **符合 Rust 惯例**：`#[cfg(target_os)]` 是标准做法
- **依赖清晰**：外部业务模块只需依赖 `rust-platform-adapter`，感知不到 workspace 结构

## 目录结构

```
rust-platform-adapter/
├── Cargo.toml                        # workspace 根配置
│
├── crates/
│   ├── adapter/                      # 核心适配库（发布到 crates.io）
│   │   ├── Cargo.toml                # name = "rust-platform-adapter"
│   │   └── src/
│   │       ├── lib.rs                # 入口，re-export 公共接口
│   │       ├── error.rs              # 统一错误类型
│   │       ├── types.rs              # 公共数据结构
│   │       │
│   │       ├── traits/               # 平台无关的 trait 定义
│   │       │   ├── mod.rs            # re-export 所有 trait
│   │       │   ├── device.rs         # 设备信息 trait
│   │       │   ├── path.rs           # 文件路径 trait
│   │       │   ├── screen.rs         # 屏幕信息 trait
│   │       │   ├── storage.rs        # 存储 trait (v1.1)
│   │       │   └── network.rs        # 网络状态 trait (v2.0)
│   │       │
│   │       └── platform/             # 平台实现
│   │           ├── mod.rs            # cfg 分发，选择当前平台实现
│   │           │
│   │           ├── android/
│   │           │   ├── mod.rs
│   │           │   ├── device.rs
│   │           │   ├── path.rs
│   │           │   └── ffi/
│   │           │
│   │           ├── ios/
│   │           │   ├── mod.rs
│   │           │   ├── device.rs
│   │           │   ├── path.rs
│   │           │   └── ffi/
│   │           │
│   │           ├── harmonyos/
│   │           │   ├── mod.rs
│   │           │   ├── device.rs
│   │           │   ├── path.rs
│   │           │   └── ffi/
│   │           │
│   │           ├── windows/
│   │           │   ├── mod.rs
│   │           │   ├── device.rs
│   │           │   ├── path.rs
│   │           │   └── ffi/
│   │           │
│   │           ├── linux/
│   │           │   ├── mod.rs
│   │           │   ├── device.rs
│   │           │   ├── path.rs
│   │           │   └── ffi/
│   │           │
│   │           └── macos/
│   │               ├── mod.rs
│   │               ├── device.rs
│   │               ├── path.rs
│   │               └── ffi/
│   │
│   └── test-app/                     # 测试应用（不发布，仅内部使用）
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
│
├── examples/                         # 简单 CLI 示例
│   ├── platform_info.rs
│   └── path_demo.rs
│
└── docs/
```

## Workspace 配置

### 根 Cargo.toml

```toml
[workspace]
members = [
    "crates/adapter",
    "crates/test-app",
]
resolver = "2"
```

### crates/adapter/Cargo.toml（核心库）

```toml
[package]
name = "rust-platform-adapter"
version = "0.1.0"
edition = "2021"
description = "Rust 跨平台适配层"
license = "MIT"

[dependencies]
# 公共依赖
thiserror = "1.0"

[target.'cfg(target_os = "android")'.dependencies]
# Android 特有依赖

[target.'cfg(target_os = "ios")'.dependencies]
# iOS 特有依赖
```

### crates/test-app/Cargo.toml（测试应用）

```toml
[package]
name = "test-app"
version = "0.1.0"
edition = "2021"
publish = false  # 不发布

[dependencies]
rust-platform-adapter = { path = "../adapter" }
```

## 核心模块职责

### `lib.rs` — 入口

```rust
mod error;
mod types;
mod traits;
mod platform;

// re-export 公共接口
pub use error::PlatformError;
pub use types::*;
pub use traits::*;
pub use platform::current::*;  // 当前平台实现
```

### `traits/` — Trait 定义

每个功能模块定义一个 trait，平台无关：

```rust
// traits/device.rs
pub trait DeviceInfo {
    fn os_type(&self) -> OsType;
    fn os_version(&self) -> &str;
    fn device_model(&self) -> &str;
    fn cpu_arch(&self) -> CpuArch;
}

// traits/path.rs
pub trait PathProvider {
    fn data_dir(&self) -> Result<PathBuf>;
    fn cache_dir(&self) -> Result<PathBuf>;
    fn temp_dir(&self) -> Result<PathBuf>;
}
```

### `platform/` — 平台实现

`platform/mod.rs` 负责 cfg 分发：

```rust
// platform/mod.rs
#[cfg(target_os = "android")]
pub mod android;

#[cfg(target_os = "ios")]
pub mod ios;

// ... 其他平台

// 重导出当前平台实现
#[cfg(target_os = "android")]
pub use android as current;

#[cfg(target_os = "ios")]
pub use ios as current;

// ... 其他平台
```

### `ffi/` — FFI 绑定

每个平台的 `ffi/` 目录存放 `extern "C"` 绑定：

```rust
// platform/android/ffi/sys.rs
extern "C" {
    fn android_get_sdk_version() -> i32;
    fn android_get_device_model(buf: *mut u8, len: usize) -> i32;
}
```

---

## 外部业务模块依赖方式

### 1. Git 依赖（未发布时）

```toml
[dependencies]
rust-platform-adapter = { git = "https://github.com/your-org/rust-platform-adapter" }
```

Cargo 会自动在 workspace 中查找 `name = "rust-platform-adapter"` 的 crate。

### 2. crates.io 依赖（正式发布后）

```toml
[dependencies]
rust-platform-adapter = "0.1.0"
```

### 3. 源码依赖（本地开发调试）

```toml
[dependencies]
rust-platform-adapter = { path = "../rust-platform-adapter/crates/adapter" }
```

### 依赖关系图

```
外部业务模块 (my-app)
       │
       │  Cargo.toml: rust-platform-adapter = { git = "..." }
       ▼
rust-platform-adapter (crates/adapter)
       │
       │  内部实现
       ▼
各平台 FFI 接口
```

**关键点**：
- 外部只需依赖 `rust-platform-adapter`，感知不到 workspace 结构
- `test-app` 是内部测试用，不影响外部依赖
- `test-app` 用路径依赖：`rust-platform-adapter = { path = "../adapter" }`

---

## 扩展指南

### 新增平台特性（如：添加剪贴板功能）

1. 创建 trait 文件 `traits/clipboard.rs`
2. 在 `traits/mod.rs` 中添加 `pub mod clipboard;`
3. 在各平台目录创建实现文件 `platform/*/clipboard.rs`
4. 在各平台 `mod.rs` 中添加 `pub mod clipboard;`

### 新增平台（如：添加 FreeBSD 支持）

1. 创建平台目录 `platform/freebsd/`
2. 在 `platform/mod.rs` 添加 cfg 分发
3. 实现所有 trait

---

## 测试应用（test-app）

### 用途

- 验证适配层功能
- 手动测试各平台表现
- 演示用法

### 运行方式

```bash
# 运行测试应用
cargo run -p test-app

# 运行示例
cargo run --example platform_info
```

### test-app 示例代码

```rust
// crates/test-app/src/main.rs
use rust_platform_adapter::prelude::*;

fn main() -> Result<()> {
    // 测试平台信息
    let info = platform_info()?;
    println!("OS: {} {}", info.os_type(), info.os_version());
    println!("Device: {}", info.device_model());

    // 测试路径
    println!("Data dir: {:?}", data_dir()?);
    println!("Cache dir: {:?}", cache_dir()?);

    Ok(())
}
```
