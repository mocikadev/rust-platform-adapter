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
    // dirs::document_dir() 依赖 XDG 配置，某些 CI 环境可能无此目录
    if let Ok(dir) = dir {
        assert!(dir.is_absolute(), "document_dir should be absolute path");
    } else {
        eprintln!("document_dir: 不可用（CI 环境可能缺少 XDG 文档目录配置）");
    }
}

#[test]
fn test_screen_info() {
    let result = screen_info();

    if let Ok(screen) = result {
        assert!(screen.width > 0, "screen width should be > 0");
        assert!(screen.height > 0, "screen height should be > 0");
        assert!(screen.dpi > 0.0, "screen dpi should be > 0");
        assert!(screen.dpi <= 1000.0, "screen dpi should be <= 1000");
        assert!(screen.scale_factor > 0.0, "scale_factor should be > 0");
        assert!(screen.scale_factor <= 4.0, "scale_factor should be <= 4");
    } else {
        eprintln!(
            "screen_info: 不可用（CI 无头环境或非主线程，错误: {:?}）",
            result.err()
        );
    }
}

#[test]
fn test_screen_width_height() {
    let w = screen_width();
    let h = screen_height();
    if let (Ok(w), Ok(h)) = (w, h) {
        assert!(w > 0, "screen width should be > 0");
        assert!(h > 0, "screen height should be > 0");
    } else {
        eprintln!("screen_width/height: 不可用（CI 无头环境或非主线程）");
    }
}

#[test]
fn test_screen_orientation() {
    if let Ok(orient) = orientation() {
        match orient {
            Orientation::Portrait | Orientation::Landscape | Orientation::Unknown => {}
        }
    } else {
        eprintln!("orientation: 不可用（CI 无头环境或非主线程）");
    }
}

#[test]
fn test_device_form() {
    let form = device_form();
    // 桌面 CI 环境应返回 Desktop
    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
    assert_eq!(
        form,
        DeviceForm::Desktop,
        "desktop CI should report DeviceForm::Desktop, got: {:?}",
        form
    );
}

#[test]
fn test_device_form_all_methods() {
    let form = device_form();
    // 桌面环境下验证返回值
    assert!(!form.is_phone(), "desktop should not be phone");
    assert!(!form.is_tablet(), "desktop should not be tablet");
    assert!(form.is_desktop(), "desktop should be desktop");
    assert!(!form.is_tv(), "desktop should not be tv");
    assert!(!form.is_car(), "desktop should not be car");
    assert!(!form.is_wearable(), "desktop should not be wearable");
    assert!(!form.is_iot(), "desktop should not be iot");
    assert!(!form.is_mobile(), "desktop should not be mobile");
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
    let form = current_device_form();
    assert_eq!(
        form,
        device_form(),
        "current_device_form should equal device_form"
    );

    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
    {
        assert!(!is_phone(), "desktop should not be phone");
        assert!(!is_tablet(), "desktop should not be tablet");
        assert!(is_desktop_device(), "desktop should be desktop_device");
    }
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

// ========== 值域校验测试 ==========

#[test]
fn test_os_version_format() {
    let version = os_version();
    assert!(version.is_ok(), "os_version should succeed");
    let version = version.unwrap();
    assert!(!version.is_empty(), "os_version should not be empty");
    // 版本号应包含数字（格式如 "24.04", "14.0", "10.0" 等）
    assert!(
        version.chars().any(|c| c.is_ascii_digit()),
        "os_version should contain digits, got: '{}'",
        version
    );
}

#[test]
fn test_data_dir_value_validation() {
    let dir = data_dir().expect("data_dir should succeed");
    assert!(dir.is_absolute(), "data_dir should be absolute");
    assert_ne!(dir.as_os_str(), "/", "data_dir should not be root");
    assert_ne!(
        dir,
        std::path::PathBuf::new(),
        "data_dir should not be empty"
    );
}

#[test]
fn test_cache_dir_value_validation() {
    let dir = cache_dir().expect("cache_dir should succeed");
    assert!(dir.is_absolute(), "cache_dir should be absolute");
    assert_ne!(dir.as_os_str(), "/", "cache_dir should not be root");
}

#[test]
fn test_temp_dir_value_validation() {
    let dir = temp_dir().expect("temp_dir should succeed");
    assert!(dir.is_absolute(), "temp_dir should be absolute");
    assert!(dir.exists(), "temp_dir should exist on filesystem");
}

#[test]
fn test_device_model_not_placeholder() {
    let model = device_model().expect("device_model should succeed");
    assert!(!model.is_empty(), "device_model should not be empty");
    let lower = model.to_lowercase();
    assert_ne!(lower, "unknown", "device_model should not be 'Unknown'");
    assert_ne!(lower, "n/a", "device_model should not be 'N/A'");
}

#[test]
fn test_platform_info_consistency() {
    let info = platform_info().expect("platform_info should succeed");
    // 验证 platform_info 各字段与单独调用一致
    assert_eq!(info.os_type, current_os());
    assert_eq!(info.cpu_arch, CpuArch::current());
    assert_eq!(info.device_form, device_form());
    assert_eq!(
        info.os_version,
        os_version().expect("os_version should succeed")
    );
    assert_eq!(
        info.device_model,
        device_model().expect("device_model should succeed")
    );
}

// ========== 异步接口真实平台测试 ==========

#[test]
fn test_async_device_info_on_platform() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let info = rt.block_on(platform_info_async());
    assert!(info.is_ok(), "platform_info_async should succeed");
    let info = info.unwrap();
    assert_eq!(info.os_type, current_os());

    let version = rt.block_on(os_version_async());
    assert!(version.is_ok(), "os_version_async should succeed");
    assert!(!version.unwrap().is_empty());

    let model = rt.block_on(device_model_async());
    assert!(model.is_ok(), "device_model_async should succeed");
    assert!(!model.unwrap().is_empty());
}

#[test]
fn test_async_path_on_platform() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let data = rt.block_on(data_dir_async());
    assert!(data.is_ok(), "data_dir_async should succeed");
    assert!(data.unwrap().is_absolute());

    let cache = rt.block_on(cache_dir_async());
    assert!(cache.is_ok(), "cache_dir_async should succeed");
    assert!(cache.unwrap().is_absolute());

    let temp = rt.block_on(temp_dir_async());
    assert!(temp.is_ok(), "temp_dir_async should succeed");
    assert!(temp.unwrap().is_absolute());

    let doc = rt.block_on(document_dir_async());
    // document_dir 在某些 CI 环境可能不可用（缺少 XDG 配置）
    if let Ok(doc) = doc {
        assert!(doc.is_absolute());
    }

    let ext_data = rt.block_on(external_data_dir_async());
    assert!(ext_data.is_ok(), "external_data_dir_async should succeed");

    let ext_cache = rt.block_on(external_cache_dir_async());
    assert!(ext_cache.is_ok(), "external_cache_dir_async should succeed");
}

#[test]
fn test_async_screen_on_platform() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let screen = rt.block_on(screen_info_async());
    if let Ok(s) = screen {
        assert!(s.width > 0);
        assert!(s.height > 0);
    } else {
        eprintln!("screen_info_async: 不可用（CI 无头环境或非主线程）");
    }
}
