use crate::error::Result;
use crate::traits::ScreenProvider;
use crate::types::{Orientation, ScreenInfo};

pub struct LinuxScreenProvider;

impl ScreenProvider for LinuxScreenProvider {
    fn screen_info(&self) -> Result<ScreenInfo> {
        // Linux 屏幕信息获取需要 X11/Wayland 依赖
        // v1.0 先返回默认值，后续可通过 x11 crate 实现
        Ok(ScreenInfo {
            width: 1920,
            height: 1080,
            dpi: 96.0,
            scale_factor: 1.0,
            orientation: Orientation::Landscape,
        })
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
