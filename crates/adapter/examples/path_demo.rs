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

    let data = rt.block_on(data_dir_async()).unwrap();
    println!("async data_dir: {}", data.display());

    let cache = rt.block_on(cache_dir_async()).unwrap();
    println!("async cache_dir: {}", cache.display());

    let screen = rt.block_on(screen_info_async()).unwrap();
    println!(
        "async screen: {}x{}, scale={:.1}",
        screen.width, screen.height, screen.scale_factor
    );
}
