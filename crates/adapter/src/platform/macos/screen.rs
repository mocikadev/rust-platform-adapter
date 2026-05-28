use crate::error::{PlatformError, Result};
use crate::traits::ScreenProvider;
use crate::types::{Orientation, ScreenInfo};

pub struct MacosScreenProvider;

impl ScreenProvider for MacosScreenProvider {
    fn screen_info(&self) -> Result<ScreenInfo> {
        #[cfg(target_os = "macos")]
        {
            use objc2::MainThreadMarker;
            use objc2_app_kit::NSScreen;

            let mtm = MainThreadMarker::new()
                .ok_or_else(|| PlatformError::FfiError("Not on main thread".to_string()))?;

            let screen = NSScreen::mainScreen(mtm)
                .ok_or_else(|| PlatformError::FfiError("No main screen".to_string()))?;
            let frame = screen.frame();
            let backing_scale = screen.backingScaleFactor();

            // NSScreen.frame 返回的点坐标（逻辑像素）
            // 乘以 backingScaleFactor 得到物理像素
            let scale = backing_scale as f32;
            let width = (frame.size.width * backing_scale) as u32;
            let height = (frame.size.height * backing_scale) as u32;

            // macOS 标准 DPI: 72.0 * backingScaleFactor
            let dpi = 72.0 * scale;

            let orientation = if width >= height {
                Orientation::Landscape
            } else {
                Orientation::Portrait
            };

            Ok(ScreenInfo {
                width,
                height,
                dpi,
                scale_factor: scale,
                orientation,
            })
        }
        #[cfg(not(target_os = "macos"))]
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
