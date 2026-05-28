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
    // 编译时常量方法验证
    assert!(OsType::Android.is_android());
    assert!(OsType::Ios.is_ios());
    assert!(OsType::Ohos.is_ohos());
    assert!(OsType::Windows.is_windows());
    assert!(OsType::Linux.is_linux());
    assert!(OsType::MacOS.is_macos());

    // is_mobile / is_desktop
    assert!(OsType::Android.is_mobile());
    assert!(OsType::Ios.is_mobile());
    assert!(!OsType::Ohos.is_mobile()); // ohos 不在 mobile 判断中
    assert!(!OsType::Linux.is_mobile());

    assert!(OsType::Windows.is_desktop());
    assert!(OsType::Linux.is_desktop());
    assert!(OsType::MacOS.is_desktop());
    assert!(!OsType::Ohos.is_desktop()); // ohos 不在 desktop 判断中
    assert!(!OsType::Android.is_desktop());
}

#[test]
fn test_device_form_methods() {
    // is_mobile 包含 Phone + Tablet
    assert!(DeviceForm::Phone.is_mobile());
    assert!(DeviceForm::Tablet.is_mobile());
    assert!(!DeviceForm::Desktop.is_mobile());
    assert!(!DeviceForm::Tv.is_mobile());

    // 各 is_xxx 方法
    assert!(DeviceForm::Phone.is_phone());
    assert!(DeviceForm::Tablet.is_tablet());
    assert!(DeviceForm::Desktop.is_desktop());
    assert!(DeviceForm::Tv.is_tv());
    assert!(DeviceForm::Car.is_car());
    assert!(DeviceForm::Wearable.is_wearable());
    assert!(DeviceForm::IoT.is_iot());
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
    assert!(
        !info.os_version.is_empty(),
        "os_version should not be empty"
    );
    assert!(
        !info.device_model.is_empty(),
        "device_model should not be empty"
    );
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
    // 某些 CI 环境（如无头 Linux）可能没有文档目录
    if let Ok(dir) = dir {
        assert!(dir.is_absolute(), "document_dir should be absolute path");
    }
}

#[test]
fn test_screen_info() {
    let screen = screen_info();
    // Linux CI 中 Xvfb 可能不稳定，X11 连接可能失败
    // 非 Linux 平台屏幕信息必须成功
    #[cfg(not(target_os = "linux"))]
    {
        let screen = screen.expect("screen_info should succeed");
        assert!(screen.width > 0, "screen width should be > 0");
        assert!(screen.height > 0, "screen height should be > 0");
        assert!(screen.dpi > 0.0, "screen dpi should be > 0");
        assert!(screen.scale_factor > 0.0, "scale_factor should be > 0");
        assert!(screen.scale_factor <= 4.0, "scale_factor should be <= 4");
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(screen) = screen {
            assert!(screen.width > 0, "screen width should be > 0");
            assert!(screen.height > 0, "screen height should be > 0");
            assert!(screen.dpi > 0.0, "screen dpi should be > 0");
            assert!(screen.scale_factor > 0.0, "scale_factor should be > 0");
        }
        // X11 连接失败时跳过（无头或 Xvfb 不稳定）
    }
}

#[test]
fn test_screen_width_height() {
    let width = screen_width();
    let height = screen_height();
    #[cfg(not(target_os = "linux"))]
    {
        let width = width.expect("screen_width should succeed");
        let height = height.expect("screen_height should succeed");
        assert!(width > 0);
        assert!(height > 0);
    }
    #[cfg(target_os = "linux")]
    {
        if let (Ok(w), Ok(h)) = (width, height) {
            assert!(w > 0);
            assert!(h > 0);
        }
    }
}

#[test]
fn test_screen_orientation() {
    let orient = orientation();
    #[cfg(not(target_os = "linux"))]
    {
        let orient = orient.expect("orientation should succeed");
        match orient {
            Orientation::Portrait | Orientation::Landscape | Orientation::Unknown => {}
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(orient) = orient {
            match orient {
                Orientation::Portrait | Orientation::Landscape | Orientation::Unknown => {}
            }
        }
    }
}

#[test]
fn test_device_form() {
    let form = device_form();
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
fn test_device_form_all_methods() {
    let form = device_form();
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
    assert!(is_android() || is_ios() || is_ohos() || is_windows() || is_linux() || is_macos());
}

#[test]
fn test_current_os_constant() {
    assert_eq!(CURRENT_OS, current_os());
}

#[test]
fn test_current_arch_constant() {
    assert_eq!(CURRENT_ARCH, CpuArch::current());
}

#[test]
fn test_os_type_current() {
    let os = OsType::current();
    assert_eq!(os, current_os());
}

#[test]
fn test_device_model_not_empty() {
    let model = device_model();
    assert!(model.is_ok(), "device_model should succeed");
    let model = model.unwrap();
    assert!(!model.is_empty(), "device_model should not be empty");
}

#[test]
fn test_os_version_not_empty() {
    let version = os_version();
    assert!(version.is_ok(), "os_version should succeed");
    let version = version.unwrap();
    assert!(!version.is_empty(), "os_version should not be empty");
}

#[test]
fn test_external_data_dir() {
    let dir = external_data_dir();
    assert!(dir.is_ok(), "external_data_dir should succeed");

    let dir = dir.unwrap();
    assert!(
        dir.is_absolute(),
        "external_data_dir should be absolute path"
    );
}

#[test]
fn test_external_cache_dir() {
    let dir = external_cache_dir();
    assert!(dir.is_ok(), "external_cache_dir should succeed");

    let dir = dir.unwrap();
    assert!(
        dir.is_absolute(),
        "external_cache_dir should be absolute path"
    );
}
