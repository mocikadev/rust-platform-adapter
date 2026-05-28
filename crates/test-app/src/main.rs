use rust_platform_adapter::prelude::*;

fn main() -> Result<()> {
    println!("=== 平台信息 ===");
    println!("OS: {:?}", current_os());
    println!("设备形态: {:?}", device_form());

    let info = platform_info()?;
    println!("系统版本: {}", info.os_version);
    println!("设备型号: {}", info.device_model);
    println!("CPU 架构: {:?}", info.cpu_arch);

    println!("\n=== 路径信息 ===");
    println!("数据目录: {:?}", data_dir()?);
    println!("缓存目录: {:?}", cache_dir()?);
    println!("临时目录: {:?}", temp_dir()?);
    println!("文档目录: {:?}", document_dir()?);

    println!("\n=== 屏幕信息 ===");
    let screen = screen_info()?;
    println!("分辨率: {}x{}", screen.width, screen.height);
    println!("DPI: {}", screen.dpi);
    println!("缩放因子: {}", screen.scale_factor);

    Ok(())
}
