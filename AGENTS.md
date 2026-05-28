# rust-platform-adapter

> 此文件是 AI 代理的项目导航地图。详细规范查阅 `docs/` 目录。

## 项目概览

**rust-platform-adapter** — Rust 跨平台适配层，统一接口调用各平台 Native C/C++ API，实现零开销抽象。  
仓库：`rust-platform-adapter`  
当前状态：**规划阶段 / 设计中**

## 技术栈

- Rust（核心语言）
- FFI（`extern "C"` 调用各平台 C/C++ 接口）
- 条件编译（`#[cfg(target_os)]` 平台分发）

## 开发流程约束

> ⚠️ **高优先级：必须按阶段推进，不得跨阶段直接动手，即使用户催促。**

```
需求 → 设计（架构 + 接口）→ 开发 → 测试 → 发布
```

- 有需求文档 → 必须先完成设计文档（`docs/design/`），才能开始开发
- 有设计文档 → 必须经过确认，才能进入开发阶段
- 开发完成 → 必须有测试覆盖（`docs/quality/`），才能进入发布流程
- 详细流程规范：`docs/process/`

## 提交前检查清单

```bash
cargo fmt --check           # 格式检查
cargo clippy -- -D warnings # lint 检查
cargo test                  # 运行测试
cargo build --target xxx    # 交叉编译验证
```

> ⚠️ 交叉编译需提前安装目标平台工具链

## 关键约束

- **平台接口一致性**：所有平台实现必须满足统一 trait 定义，行为一致
- **安全边界**：FFI 调用必须包裹在 `unsafe` 块中，需添加安全注释说明
- **Feature Flag 规范**：平台选择通过 `cfg` 属性，不允许运行时动态切换
- 跨平台接口变更时，必须同步更新所有平台实现

## 常用命令

```bash
cargo build                     # 默认平台构建
cargo build --target aarch64-linux-android  # Android 交叉编译
cargo build --target aarch64-apple-ios      # iOS 交叉编译
cargo test                      # 运行测试
cargo doc --open                # 生成并查看文档
```

## 文档导航

| 文档 | 路径 |
|------|------|
| 需求文档 | `docs/requirements/` |
| 技术设计 | `docs/design/` |
| 开发记录 | `docs/development/` |
| Bug 修订 | `docs/bugfix/` |
| 提交规范 | `docs/process/` |
| 代码风格 | `docs/style/` |
| 测试策略 | `docs/quality/` |

## Skills 导航

| Skill | 说明 |
|-------|------|
| `rust-best-practices` | Rust 编码最佳实践指南 |
| `rust-skills` | Rust 编码规范与反模式 |
