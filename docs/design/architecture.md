# 架构设计

## 整体架构

```
┌─────────────────────────────────────────────┐
│           Rust 业务逻辑层                    │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│         Rust 平台适配层 (trait)               │
│         统一接口定义                          │
└──────────────────┬──────────────────────────┘
                   │
   ┌───────────────┼───────────────┐
   │       #[cfg(target_os)]       │
   ▼               ▼               ▼
┌──────┐      ┌──────┐       ┌──────┐
│Rust  │      │Rust  │       │Rust  │  ...
│代码  │      │代码  │       │代码  │
└──┬───┘      └──┬───┘       └──┬───┘
   │             │              │
   ▼             ▼              ▼
Android NDK   iOS ObjC      Win32 API
   C API       C API          C API
```

## 核心设计原则

1. **零开销抽象**：编译期静态分发，无运行时开销
2. **统一接口**：trait 定义平台无关 API
3. **条件编译**：`#[cfg(target_os)]` 选择平台实现
4. **FFI 直调**：直接调用各平台 C/C++ 接口，无中间层

## 目录结构

```
rust-platform-adapter/
├── Cargo.toml
├── src/
│   ├── lib.rs              # 入口 + trait 定义
│   ├── platform/
│   │   ├── mod.rs          # 平台分发
│   │   ├── android.rs      # Android 实现
│   │   ├── ios.rs          # iOS 实现
│   │   ├── harmonyos.rs    # HarmonyOS 实现
│   │   ├── windows.rs      # Windows 实现
│   │   ├── linux.rs        # Linux 实现
│   │   └── macos.rs        # macOS 实现
│   └── types.rs            # 公共类型定义
```

## Trait 设计示例

```rust
pub trait PlatformAdapter {
    fn platform_name(&self) -> &str;
    fn device_info(&self) -> Result<DeviceInfo>;
    // ... 其他平台能力
}
```
