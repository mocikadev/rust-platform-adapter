use crate::error::{PlatformError, Result};
use crate::traits::ScreenProvider;
use crate::types::{Orientation, ScreenInfo};

pub struct IosScreenProvider;

impl ScreenProvider for IosScreenProvider {
    fn screen_info(&self) -> Result<ScreenInfo> {
        #[cfg(target_os = "ios")]
        {
            use objc2::MainThreadMarker;
            use objc2_ui_kit::{UIDevice, UIDeviceOrientation, UIScreen, UIUserInterfaceIdiom};

            let mtm = MainThreadMarker::new()
                .ok_or_else(|| PlatformError::FfiError("Not on main thread".to_string()))?;

            let screen = UIScreen::mainScreen(mtm);
            let bounds = screen.bounds();
            let scale = screen.scale();

            // bounds.size 返回的是逻辑点(points)，必须乘以 scale 得到物理像素
            let width = (bounds.size.width * scale) as u32;
            let height = (bounds.size.height * scale) as u32;

            // 使用 UIDevice.orientation 获取设备物理方向
            // 注意：UIScreen.bounds 始终返回固定值，不随旋转变化
            let device = UIDevice::currentDevice(mtm);
            let orientation = match device.orientation() {
                UIDeviceOrientation::Portrait | UIDeviceOrientation::PortraitUpsideDown => {
                    Orientation::Portrait
                }
                UIDeviceOrientation::LandscapeLeft | UIDeviceOrientation::LandscapeRight => {
                    Orientation::Landscape
                }
                // FaceUp/FaceDown/Unknown 时回退到 bounds 尺寸推断
                _ => {
                    if width >= height {
                        Orientation::Landscape
                    } else {
                        Orientation::Portrait
                    }
                }
            };

            // 根据 userInterfaceIdiom 区分设备类型使用不同基准 PPI
            let idiom = device.userInterfaceIdiom();
            let base_ppi = match idiom {
                UIUserInterfaceIdiom::Phone => 163.0,
                UIUserInterfaceIdiom::Pad => 132.0,
                _ => 163.0,
            };

            Ok(ScreenInfo {
                width,
                height,
                dpi: (scale * base_ppi) as f32,
                scale_factor: scale as f32,
                orientation,
            })
        }
        #[cfg(not(target_os = "ios"))]
        {
            Err(PlatformError::NotSupported)
        }
    }

    fn screen_width(&self) -> Result<u32> {
        Ok(self.screen_info()?.width)
    }

    fn screen_height(&self) -> Result<u32> {
        Ok(self.screen_info()?.height)
    }

    fn scale_factor(&self) -> Result<f32> {
        Ok(self.screen_info()?.scale_factor)
    }

    fn orientation(&self) -> Result<Orientation> {
        Ok(self.screen_info()?.orientation)
    }
}
