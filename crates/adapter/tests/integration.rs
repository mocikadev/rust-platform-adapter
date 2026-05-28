use rust_platform_adapter::prelude::*;

// ========== Mock 实现 ==========

/// Mock DeviceInfo 实现
struct MockDevice;

impl DeviceInfo for MockDevice {
    fn platform_info(&self) -> Result<PlatformInfo> {
        Ok(PlatformInfo {
            os_type: OsType::Linux,
            os_version: "1.0.0-mock".to_string(),
            device_model: "MockDevice".to_string(),
            cpu_arch: CpuArch::X86_64,
            device_form: DeviceForm::Desktop,
        })
    }

    fn os_type(&self) -> OsType {
        OsType::Linux
    }

    fn os_version(&self) -> Result<String> {
        Ok("1.0.0-mock".to_string())
    }

    fn device_model(&self) -> Result<String> {
        Ok("MockDevice".to_string())
    }

    fn cpu_arch(&self) -> CpuArch {
        CpuArch::X86_64
    }

    fn device_form(&self) -> DeviceForm {
        DeviceForm::Desktop
    }
}

/// Mock PathProvider 实现
struct MockPath;

impl PathProvider for MockPath {
    fn data_dir(&self) -> Result<std::path::PathBuf> {
        Ok(std::path::PathBuf::from("/mock/data"))
    }

    fn cache_dir(&self) -> Result<std::path::PathBuf> {
        Ok(std::path::PathBuf::from("/mock/cache"))
    }

    fn temp_dir(&self) -> Result<std::path::PathBuf> {
        Ok(std::path::PathBuf::from("/mock/temp"))
    }

    fn document_dir(&self) -> Result<std::path::PathBuf> {
        Ok(std::path::PathBuf::from("/mock/documents"))
    }

    fn external_data_dir(&self) -> Result<std::path::PathBuf> {
        Ok(std::path::PathBuf::from("/mock/external/data"))
    }

    fn external_cache_dir(&self) -> Result<std::path::PathBuf> {
        Ok(std::path::PathBuf::from("/mock/external/cache"))
    }
}

/// Mock ScreenProvider 实现
struct MockScreen;

impl ScreenProvider for MockScreen {
    fn screen_info(&self) -> Result<ScreenInfo> {
        Ok(ScreenInfo {
            width: 1920,
            height: 1080,
            dpi: 96.0,
            scale_factor: 1.0,
            orientation: Orientation::Landscape,
        })
    }

    fn screen_width(&self) -> Result<u32> {
        Ok(1920)
    }

    fn screen_height(&self) -> Result<u32> {
        Ok(1080)
    }

    fn scale_factor(&self) -> Result<f32> {
        Ok(1.0)
    }

    fn orientation(&self) -> Result<Orientation> {
        Ok(Orientation::Landscape)
    }
}

// ========== 原有集成测试 ==========

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
    // 所有平台的 CI 环境都可能无头（无显示器），屏幕信息获取可能失败
    if let Ok(screen) = screen_info() {
        assert!(screen.width > 0, "screen width should be > 0");
        assert!(screen.height > 0, "screen height should be > 0");
        assert!(screen.dpi > 0.0, "screen dpi should be > 0");
        assert!(screen.scale_factor > 0.0, "scale_factor should be > 0");
        assert!(screen.scale_factor <= 4.0, "scale_factor should be <= 4");
    }
    // CI 无头环境跳过
}

#[test]
fn test_screen_width_height() {
    if let (Ok(w), Ok(h)) = (screen_width(), screen_height()) {
        assert!(w > 0);
        assert!(h > 0);
    }
}

#[test]
fn test_screen_orientation() {
    if let Ok(orient) = orientation() {
        match orient {
            Orientation::Portrait | Orientation::Landscape | Orientation::Unknown => {}
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

#[test]
fn test_device_form_convenience_functions() {
    // 验证新增的便捷函数可调用
    let form = current_device_form();
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
    // is_phone / is_tablet / is_desktop_device 应返回布尔值
    let _ = is_phone();
    let _ = is_tablet();
    let _ = is_desktop_device();
}

// ========== Mock 实现测试 ==========

#[test]
fn test_mock_device_info() {
    let device = MockDevice;
    assert_eq!(device.os_type(), OsType::Linux);
    assert_eq!(device.os_version().unwrap(), "1.0.0-mock");
    assert_eq!(device.device_model().unwrap(), "MockDevice");
    assert_eq!(device.cpu_arch(), CpuArch::X86_64);
    assert_eq!(device.device_form(), DeviceForm::Desktop);

    let info = device.platform_info().unwrap();
    assert_eq!(info.os_type, OsType::Linux);
    assert_eq!(info.os_version, "1.0.0-mock");
    assert_eq!(info.device_model, "MockDevice");
    assert_eq!(info.cpu_arch, CpuArch::X86_64);
    assert_eq!(info.device_form, DeviceForm::Desktop);
}

#[test]
fn test_mock_path_provider() {
    let path = MockPath;
    assert_eq!(
        path.data_dir().unwrap(),
        std::path::PathBuf::from("/mock/data")
    );
    assert_eq!(
        path.cache_dir().unwrap(),
        std::path::PathBuf::from("/mock/cache")
    );
    assert_eq!(
        path.temp_dir().unwrap(),
        std::path::PathBuf::from("/mock/temp")
    );
    assert_eq!(
        path.document_dir().unwrap(),
        std::path::PathBuf::from("/mock/documents")
    );
    assert_eq!(
        path.external_data_dir().unwrap(),
        std::path::PathBuf::from("/mock/external/data")
    );
    assert_eq!(
        path.external_cache_dir().unwrap(),
        std::path::PathBuf::from("/mock/external/cache")
    );
}

#[test]
fn test_mock_screen_provider() {
    let screen = MockScreen;
    assert_eq!(screen.screen_width().unwrap(), 1920);
    assert_eq!(screen.screen_height().unwrap(), 1080);
    assert_eq!(screen.scale_factor().unwrap(), 1.0);
    assert_eq!(screen.orientation().unwrap(), Orientation::Landscape);

    let info = screen.screen_info().unwrap();
    assert_eq!(info.width, 1920);
    assert_eq!(info.height, 1080);
    assert_eq!(info.dpi, 96.0);
    assert_eq!(info.scale_factor, 1.0);
    assert_eq!(info.orientation, Orientation::Landscape);
}

#[test]
fn test_mock_async_interfaces() {
    let device = MockDevice;
    let path = MockPath;
    let screen = MockScreen;

    // 验证异步接口可调用且返回正确结果
    let rt = tokio::runtime::Runtime::new().unwrap();

    let device_info = rt.block_on(device.platform_info_async()).unwrap();
    assert_eq!(device_info.os_type, OsType::Linux);

    let data = rt.block_on(path.data_dir_async()).unwrap();
    assert_eq!(data, std::path::PathBuf::from("/mock/data"));

    let screen_info = rt.block_on(screen.screen_info_async()).unwrap();
    assert_eq!(screen_info.width, 1920);
}
