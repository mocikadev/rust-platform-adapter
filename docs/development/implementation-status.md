# 实现状态

## 各平台实现状态

| 平台 | 状态 | 实现方式 | 验证环境 |
|------|------|----------|----------|
| Linux | ✅ 完成 | `dirs` + `sysinfo` + `x11rb` | 当前环境已验证 |
| Windows | ✅ 完成 | `dirs` + `sysinfo` + `windows` crate | CI (GitHub Actions) |
| macOS | ✅ 完成 | `dirs` + `sysinfo` + `objc2-app-kit` + `libc` | CI (GitHub Actions) |
| iOS | ✅ 完成 | `objc2` FFI | 代码完成，需 iOS 设备验证 |
| Android | ✅ 完成 | `ndk` FFI + `/proc/self/cmdline` | 代码完成，需 Android 设备验证 |
| OpenHarmony | ✅ 完成 | `extern "C"` FFI | 代码完成，需 OpenHarmony 设备验证 |

## 实现详情

### Linux

- **DeviceInfo**: 使用 `sysinfo` 获取系统信息，读取 `/sys/class/dmi/id/product_name` 获取设备型号
- **PathProvider**: 使用 `dirs` 库获取标准目录
- **ScreenProvider**: 使用 `x11rb` + RandR 扩展获取真实屏幕信息（分辨率、DPI、缩放因子）
  - 检测 `DISPLAY` 环境变量判断 X11 可用性
  - 通过 RandR `get_output_info` 获取显示器物理尺寸计算 DPI
  - Wayland 环境返回 `NotSupported`（Wayland 安全模型限制）
  - 无显示服务器时返回 `NotSupported`

### Windows

- **DeviceInfo**: 使用 `sysinfo` 获取系统信息，通过注册表 `HKEY_LOCAL_MACHINE\HARDWARE\DESCRIPTION\System\BIOS\SystemProductName` 获取真实设备型号
- **PathProvider**: 使用 `dirs` 库获取标准目录
- **ScreenProvider**: 使用 `windows` crate 调用 Win32 API
  - `GetSystemMetrics(SM_CXSCREEN/SM_CYSCREEN)` 获取分辨率
  - `GetDeviceCaps(LOGPIXELSX)` 获取 DPI

### macOS

- **DeviceInfo**: 使用 `sysinfo` 获取系统信息，通过 `libc::sysctlbyname("hw.model")` 获取真实设备型号（如 "MacBookPro18,3"）
- **PathProvider**: 使用 `dirs` 库获取标准目录
- **ScreenProvider**: 使用 `objc2-app-kit` 调用 `NSScreen` API
  - `NSScreen.mainScreen().frame()` 获取分辨率（逻辑像素 × backingScaleFactor = 物理像素）
  - `NSScreen.mainScreen().backingScaleFactor()` 获取缩放因子

### iOS

- **DeviceInfo**: 使用 `objc2-ui-kit` 调用 `UIDevice`
- **PathProvider**: 使用 `objc2-foundation` 调用 `NSSearchPathForDirectoriesInDomains`
- **ScreenProvider**: 使用 `objc2-ui-kit` 调用 `UIScreen` + `UIApplication`
  - 根据 `UIDevice.userInterfaceIdiom` 区分 iPhone (163 PPI) / iPad (132 PPI) 计算 DPI

### Android

- **DeviceInfo**: 使用 `ndk` crate 的 `Configuration` + raw `extern "C"` 声明 `__system_property_get` 获取系统属性
- **PathProvider**: 读取 `/proc/self/cmdline` 获取包名，拼接真实路径（如 `/data/data/com.example.app/files`）
- **ScreenProvider**: 使用 `ndk` crate 的 `Configuration` 获取屏幕信息（方向、密度），通过 dp × density 估算像素尺寸

### OpenHarmony

使用 OpenHarmony NDK 官方 C API，无需 `ohos-rs` 框架。

- **DeviceInfo**: 使用 `deviceinfo.h` 的 `OH_GetDeviceType()`, `OH_GetProductModel()`, `OH_GetOSFullName()`
- **PathProvider**: 使用 `application_context.h` 的 `OH_AbilityRuntime_ApplicationContextGet*Dir()`
- **ScreenProvider**: 使用 `oh_display_manager.h` 的 `OH_NativeDisplayManager_GetDefaultDisplay*()`

**依赖的 OpenHarmony NDK 库**:
- `libdeviceinfo_ndk.z.so` — 设备信息
- `libability_runtime.so` — 应用上下文（路径）
- `libnative_display_manager.so` — 屏幕信息

## CI/CD

### CI Workflow (每次 push/PR)

- **fmt**: `cargo fmt --check` (ubuntu-latest)
- **clippy**: `cargo clippy -- -D warnings` (ubuntu-latest)
- **test**: `cargo test` (Linux + Windows + macOS 三平台)
- **check**: `cargo check --all-targets` (Linux + Windows + macOS 三平台)
- Linux 使用 Xvfb 虚拟 X 服务器测试屏幕信息获取

### Build Workflow (push main / release)

- **Desktop**: Linux/Windows/macOS 多架构构建
- **Android**: `cargo-ndk` 交叉编译 (arm64-v8a / armeabi-v7a / x86_64)
- **iOS**: 交叉编译 (aarch64-apple-ios / aarch64-apple-ios-sim)
- **OpenHarmony**: `setup-ohos-sdk` + 交叉编译 (aarch64-unknown-linux-ohos)

## 后续优化

1. **Linux Wayland**: 支持 Wayland 环境下屏幕信息获取（需通过 xdg-desktop-portal）
2. **Android ScreenProvider**: 通过 JNI 或 `ANativeWindow` 获取精确像素尺寸
3. **Linux DeviceForm**: 通过 `/sys/class/dmi/id/chassis_type` 判断设备形态
4. **Windows DeviceForm**: 通过 `GetSystemMetrics(SM_TABLETPC)` 判断是否为平板
