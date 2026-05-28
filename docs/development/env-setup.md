# 开发环境配置

## 一、Rust 工具链

### 1.1 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 1.2 安装交叉编译目标

```bash
# Android
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android

# iOS
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios

# OpenHarmony (需自定义工具链)
# 参考 ohos-rs 文档: https://ohos.rs/en/docs/basic/quick-start.html
```

---

## 二、Android NDK

### 2.1 当前环境

- **NDK 路径**：`/opt/android/sdk/ndk/29.0.14206865`
- **环境变量**：`ANDROID_NDK_HOME=/opt/android/sdk/ndk/29.0.14206865`
- **支持 API Level**：21-30+

### 2.2 交叉编译配置

创建 `.cargo/config.toml`：

```toml
[target.aarch64-linux-android]
linker = "/opt/android/sdk/ndk/29.0.14206865/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android29-clang"

[target.armv7-linux-androideabi]
linker = "/opt/android/sdk/ndk/29.0.14206865/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi29-clang"

[target.x86_64-linux-android]
linker = "/opt/android/sdk/ndk/29.0.14206865/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android29-clang"
```

### 2.3 验证编译

```bash
cargo build --target aarch64-linux-android
```

---

## 三、iOS 开发环境（macOS）

### 3.1 前置条件

- 安装 Xcode
- 安装 Xcode Command Line Tools

### 3.2 验证编译

```bash
cargo build --target aarch64-apple-ios
```

---

## 四、OpenHarmony 开发环境

### 4.1 安装 ohos-rs CLI

```bash
cargo install cargo-ohrs
```

### 4.2 依赖配置

```toml
[dependencies]
napi-ohos = "1.2.0"
napi-derive-ohos = "1.2.0"
```

### 4.3 构建

```bash
ohrs build
# 或指定架构
ohrs build --arch aarch64
```

---

## 五、桌面系统

Windows、Linux、macOS 无需额外配置，直接编译：

```bash
cargo build
cargo test
```

---

## 六、环境验证脚本

```bash
#!/bin/bash
echo "=== Rust 版本 ==="
rustc --version
cargo --version

echo "=== 已安装目标 ==="
rustup target list --installed

echo "=== Android NDK ==="
echo $ANDROID_NDK_HOME
ls $ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android*-clang 2>/dev/null | head -1

echo "=== 编译测试 ==="
cargo build --target aarch64-linux-android 2>&1 | tail -3
```
