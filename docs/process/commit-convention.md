# 提交规范

## Commit Message 格式

```
<type>(<scope>): <subject>

<body>

<footer>
```

## Type 类型

| 类型 | 说明 |
|------|------|
| `feat` | 新功能 |
| `fix` | Bug 修复 |
| `docs` | 文档变更 |
| `style` | 代码格式（不影响逻辑） |
| `refactor` | 重构 |
| `perf` | 性能优化 |
| `test` | 测试相关 |
| `chore` | 构建/工具变更 |

## Scope 范围

- `android` / `ios` / `harmonyos` / `windows` / `linux` / `macos` — 平台相关
- `core` — 核心接口
- `ci` — CI/CD
- `docs` — 文档

## 示例

```
feat(android): 实现设备信息获取

- 通过 NDK 获取设备型号、系统版本
- 添加对应单元测试

Closes #12
```
