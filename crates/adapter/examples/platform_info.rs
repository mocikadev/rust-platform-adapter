//! 平台信息示例
//!
//! 演示如何获取设备信息、屏幕信息和路径信息。
//!
//! ```sh
//! cargo run --example platform_info
//! ```

use rust_platform_adapter::prelude::*;

fn main() {
    println!("=== 平台信息 ===");

    // 编译时常量
    println!("操作系统: {:?}", current_os());
    println!("CPU 架构: {:?}", CpuArch::current());
    println!(
        "is_android={}, is_ios={}, is_ohos={}, is_windows={}, is_linux={}, is_macos={}",
        is_android(),
        is_ios(),
        is_ohos(),
        is_windows(),
        is_linux(),
        is_macos()
    );

    // 设备信息
    if let Ok(info) = platform_info() {
        println!(
            "平台: {:?} {} {} {:?} {:?}",
            info.os_type, info.os_version, info.device_model, info.cpu_arch, info.device_form
        );
    }

    if let Ok(version) = os_version() {
        println!("操作系统版本: {}", version);
    }

    if let Ok(model) = device_model() {
        println!("设备型号: {}", model);
    }

    let form = device_form();
    println!(
        "设备形态: {:?} (phone={}, tablet={}, desktop={})",
        form,
        form.is_phone(),
        form.is_tablet(),
        form.is_desktop()
    );

    println!();
    println!("=== 屏幕信息 ===");

    if let Ok(screen) = screen_info() {
        println!(
            "屏幕: {}x{} @ {:.0} DPI, scale={:.1}, orientation={:?}",
            screen.width, screen.height, screen.dpi, screen.scale_factor, screen.orientation
        );
    } else {
        println!("(无头环境，无法获取屏幕信息)");
    }

    println!();
    println!("=== 路径信息 ===");

    print_path("数据目录", data_dir());
    print_path("缓存目录", cache_dir());
    print_path("临时目录", temp_dir());
    print_path("文档目录", document_dir());
    print_path("外部数据目录", external_data_dir());
    print_path("外部缓存目录", external_cache_dir());
}

fn print_path(label: &str, result: Result<std::path::PathBuf>) {
    match result {
        Ok(path) => println!("{}: {}", label, path.display()),
        Err(e) => println!("{}: 错误 - {}", label, e),
    }
}
