use rust_platform_adapter::prelude::*;

#[test]
fn test_current_os() {
    let os = current_os();
    #[cfg(target_os = "linux")]
    assert_eq!(os, OsType::Linux);
    #[cfg(target_os = "windows")]
    assert_eq!(os, OsType::Windows);
    #[cfg(target_os = "macos")]
    assert_eq!(os, OsType::MacOS);
    #[cfg(target_os = "android")]
    assert_eq!(os, OsType::Android);
    #[cfg(target_os = "ios")]
    assert_eq!(os, OsType::Ios);
    #[cfg(target_os = "ohos")]
    assert_eq!(os, OsType::Ohos);
}

#[test]
fn test_os_type_methods() {
    let os = current_os();
    #[cfg(target_os = "linux")]
    assert!(os.is_linux());
    #[cfg(target_os = "windows")]
    assert!(os.is_windows());
    #[cfg(target_os = "macos")]
    assert!(os.is_macos());
    #[cfg(target_os = "android")]
    assert!(os.is_android());
    #[cfg(target_os = "ios")]
    assert!(os.is_ios());
    #[cfg(target_os = "ohos")]
    assert!(os.is_ohos());
}

#[test]
fn test_cpu_arch() {
    let arch = CpuArch::current();
    #[cfg(target_arch = "x86_64")]
    assert_eq!(arch, CpuArch::X86_64);
    #[cfg(target_arch = "x86")]
    assert_eq!(arch, CpuArch::X86);
    #[cfg(target_arch = "aarch64")]
    assert_eq!(arch, CpuArch::Arm64);
    #[cfg(target_arch = "arm")]
    assert_eq!(arch, CpuArch::Arm);
}

#[test]
fn test_platform_info() {
    let info = platform_info();
    assert!(info.is_ok(), "platform_info should succeed");

    let info = info.unwrap();
    assert_eq!(info.os_type, current_os());
    assert!(!info.os_version.is_empty(), "os_version should not be empty");
}

#[test]
fn test_data_dir() {
    let dir = data_dir();
    assert!(dir.is_ok(), "data_dir should succeed");

    let dir = dir.unwrap();
    assert!(dir.is_absolute(), "data_dir should be absolute path");
}

#[test]
fn test_cache_dir() {
    let dir = cache_dir();
    assert!(dir.is_ok(), "cache_dir should succeed");

    let dir = dir.unwrap();
    assert!(dir.is_absolute(), "cache_dir should be absolute path");
}

#[test]
fn test_temp_dir() {
    let dir = temp_dir();
    assert!(dir.is_ok(), "temp_dir should succeed");

    let dir = dir.unwrap();
    assert!(dir.is_absolute(), "temp_dir should be absolute path");
}

#[test]
fn test_document_dir() {
    let dir = document_dir();
    assert!(dir.is_ok(), "document_dir should succeed");

    let dir = dir.unwrap();
    assert!(dir.is_absolute(), "document_dir should be absolute path");
}

#[test]
fn test_screen_info() {
    let screen = screen_info();
    assert!(screen.is_ok(), "screen_info should succeed");

    let screen = screen.unwrap();
    assert!(screen.width > 0, "screen width should be > 0");
    assert!(screen.height > 0, "screen height should be > 0");
    assert!(screen.dpi > 0.0, "screen dpi should be > 0");
    assert!(screen.scale_factor > 0.0, "scale_factor should be > 0");
}

#[test]
fn test_screen_width_height() {
    let width = screen_width();
    let height = screen_height();
    assert!(width.is_ok(), "screen_width should succeed");
    assert!(height.is_ok(), "screen_height should succeed");

    let width = width.unwrap();
    let height = height.unwrap();
    assert!(width > 0, "screen width should be > 0");
    assert!(height > 0, "screen height should be > 0");
}

#[test]
fn test_device_form() {
    let form = device_form();
    // 无法断言具体值，只验证不 panic
    match form {
        DeviceForm::Phone
        | DeviceForm::Tablet
        | DeviceForm::Desktop
        | DeviceForm::Tv
        | DeviceForm::Car
        | DeviceForm::Wearable
        | DeviceForm::IoT
        | DeviceForm::Unknown => {}
    }
}

#[test]
fn test_device_form_methods() {
    let form = device_form();
    // 验证所有方法可用
    let _ = form.is_phone();
    let _ = form.is_tablet();
    let _ = form.is_desktop();
    let _ = form.is_tv();
    let _ = form.is_car();
    let _ = form.is_wearable();
    let _ = form.is_iot();
    let _ = form.is_mobile();
}

#[test]
fn test_convenience_functions() {
    // 验证所有便捷函数可用
    assert!(is_android() || is_ios() || is_ohos() || is_windows() || is_linux() || is_macos());
}
