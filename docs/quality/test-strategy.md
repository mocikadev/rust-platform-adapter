# 测试策略

## 测试层次

| 层次 | 说明 | 工具 |
|------|------|------|
| 单元测试 | 函数级别测试 | `cargo test` |
| 集成测试 | 模块间交互 | `tests/` 目录 |
| 交叉编译验证 | 确保各平台可编译 | CI + 各平台工具链 |

## 平台测试

- 使用 mock 或条件编译隔离平台依赖
- 核心 trait 使用 mock 实现进行测试
- 平台特定代码在对应平台 CI 环境验证

## 环境感知断言策略

测试中使用 `has_display()` 函数检测图形环境可用性，决定断言强度：

| 接口类别 | 有头环境 | 无头环境 |
|----------|---------|---------|
| 设备信息（os_type, os_version, device_model, device_form, cpu_arch） | 强制断言 | 不运行（移动平台 CI 仅编译） |
| 路径接口（data_dir, cache_dir, temp_dir, document_dir 等） | 强制断言 | 不运行 |
| 屏幕接口（screen_info, screen_width 等） | 强制断言 | `if let Ok` 容错 |
| 异步接口 | 强制断言（真实平台调用） | 不运行 |

### CI 环境能力矩阵

| 平台 | 虚拟显示 | 设备信息 | 路径接口 | 屏幕接口 |
|------|---------|---------|---------|---------|
| Linux (Xvfb) | :99 1920x1080x24 | /sys/class/dmi/id | dirs crate | x11rb |
| macOS | 虚拟显示器 | sysctl hw.model | dirs crate | NSScreen |
| Windows | 虚拟桌面 | 注册表 | dirs crate | Win32 GDI |
| Android | N/A | 仅编译验证 | 仅编译验证 | 仅编译验证 |
| iOS | N/A | 仅编译验证 | 仅编译验证 | 仅编译验证 |
| OpenHarmony | N/A | 仅编译验证 | 仅编译验证 | 仅编译验证 |

### 值域校验

除接口成功/失败断言外，还验证返回值的合理性：

- `os_version`: 必须包含数字字符
- 路径接口: 必须为绝对路径、非根目录、非空
- `temp_dir`: 必须在文件系统上存在
- `device_model`: 不应为 "Unknown" 或 "N/A" 占位符
- `platform_info`: 各字段应与单独调用一致

## CI 矩阵

```yaml
platforms:
  - ubuntu-latest (Linux)
  - macos-latest (macOS)
  - windows-latest (Windows)
  - ubuntu + cargo-ndk (Android)
  - macos + cargo-lipo (iOS)
```

## 验收标准

- 单元测试覆盖率 > 80%
- 所有平台交叉编译通过
- clippy 无警告
