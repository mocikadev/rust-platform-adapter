//! 路径操作示例
//!
//! 演示如何使用路径接口和异步接口。
//!
//! ```sh
//! cargo run --example path_demo
//! ```

use rust_platform_adapter::prelude::*;

fn main() {
    println!("=== 同步路径接口 ===");

    if let Ok(dir) = data_dir() {
        println!("data_dir: {}", dir.display());
        // 在数据目录下创建子目录
        let db_path = dir.join("app.db");
        println!("数据库路径: {}", db_path.display());
    }

    if let Ok(dir) = cache_dir() {
        println!("cache_dir: {}", dir.display());
    }

    // 外部存储（Android 上与内部不同，其他平台相同）
    if let Ok(dir) = external_data_dir() {
        println!("external_data_dir: {}", dir.display());
    }

    println!();
    println!("=== 异步接口 ===");

    // 异步接口适合在异步运行时中使用
    // 这里用 block_on 演示，实际应使用 .await
    let rt = tokio::runtime::Runtime::new().unwrap();

    match rt.block_on(data_dir_async()) {
        Ok(data) => println!("async data_dir: {}", data.display()),
        Err(e) => println!("async data_dir: 错误 - {}", e),
    }

    match rt.block_on(cache_dir_async()) {
        Ok(cache) => println!("async cache_dir: {}", cache.display()),
        Err(e) => println!("async cache_dir: 错误 - {}", e),
    }

    // 屏幕信息在有头环境下才可用
    if has_display() {
        if let Ok(screen) = rt.block_on(screen_info_async()) {
            println!(
                "async screen: {}x{}, scale={:.1}",
                screen.width, screen.height, screen.scale_factor
            );
        }
    } else {
        println!("(无头环境，跳过异步屏幕信息获取)");
    }
}
