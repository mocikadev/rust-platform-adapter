use crate::error::Result;
use crate::traits::ScreenProvider;
use crate::types::{Orientation, ScreenInfo};

pub struct IosScreenProvider;

impl ScreenProvider for IosScreenProvider {
    fn screen_info(&self) -> Result<ScreenInfo> {
        // iOS 使用 UIScreen 获取屏幕信息
        #[cfg(target_os = "ios")]
        {
            use objc2_ui_kit::{UIApplication, UIScreen};
            let screen = unsafe { UIScreen::mainScreen() };
            let bounds = unsafe { screen.bounds() };
            let scale = unsafe { screen.scale() };

            let width = bounds.size.width as u32;
            let height = bounds.size.height as u32;

            // 获取屏幕方向
            let orientation = unsafe {
                let app = UIApplication::sharedApplication();
                let status_bar_orientation = app.statusBarOrientation();
                match status_bar_orientation {
                    1 => Orientation::Portrait,       // UIInterfaceOrientationPortrait
                    2 => Orientation::Portrait,       // UIInterfaceOrientationPortraitUpsideDown
                    3 => Orientation::Landscape,      // UIInterfaceOrientationLandscapeLeft
                    4 => Orientation::Landscape,      // UIInterfaceOrientationLandscapeRight
                    _ => Orientation::Unknown,
                }
            };

            Ok(ScreenInfo {
                width,
                height,
                dpi: (scale * 163.0) as f32,
                scale_factor: scale as f32,
                orientation,
            })
        }
        #[cfg(not(target_os = "ios"))]
        {
            Err(crate::error::PlatformError::NotSupported)
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
