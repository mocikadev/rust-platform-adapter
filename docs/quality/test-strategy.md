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
