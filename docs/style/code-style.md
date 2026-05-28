# 代码风格

## 基本规范

- 遵循 Rust 官方风格指南
- 使用 `cargo fmt` 格式化
- 使用 `clippy` 静态分析

## FFI 规范

```rust
// FFI 函数必须包裹在 unsafe 块中
// 必须添加安全注释说明为什么是安全的
unsafe {
    // Safety: 该函数接受有效指针，生命周期由调用方保证
    let result = ffi_function(ptr);
}
```

## 命名约定

- Rust 侧：遵循 Rust 命名规范（snake_case 函数，PascalCase 类型）
- FFI 导出：使用 C 风格命名（如 `platform_get_device_info`）
- 文件名：snake_case（如 `android.rs`、`harmonyos.rs`）

## 注释规范

- 公共 API 必须有文档注释（`///`）
- FFI 函数必须有 Safety 注释
- 复杂逻辑添加行内注释
