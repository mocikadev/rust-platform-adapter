use crate::error::Result;
use crate::traits::ScreenProvider;
use crate::types::{Orientation, ScreenInfo};

pub struct MacosScreenProvider;

impl ScreenProvider for MacosScreenProvider {
    fn screen_info(&self) -> Result<ScreenInfo> {
        // macOS 屏幕信息获取需要 AppKit 框架
        // v1.0 先返回默认值，后续可通过 objc2-app-kit 实现
        // macOS 默认 DPI 是 72 (Retina 显示器为 144)
        Ok(ScreenInfo {
            width: 2560,
            height: 1600,
            dpi: 72.0,
            scale_factor: 2.0,
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
