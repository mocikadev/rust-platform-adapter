use crate::error::Result;
use crate::traits::ScreenProvider;
use crate::types::{Orientation, ScreenInfo};

pub struct AndroidScreenProvider;

impl ScreenProvider for AndroidScreenProvider {
    fn screen_info(&self) -> Result<ScreenInfo> {
        // Android 使用 NDK Configuration 获取屏幕信息
        #[cfg(target_os = "android")]
        {
            use ndk::configuration::Configuration;
            let config = Configuration::new();
            let orientation = config.orientation();

            use ndk::configuration::Orientation as NdkOrientation;
            let orientation = match orientation {
                NdkOrientation::Port => Orientation::Portrait,
                NdkOrientation::Land => Orientation::Landscape,
                _ => Orientation::Unknown,
            };

            // 屏幕尺寸需要通过 JNI 或 ANativeWindow 获取
            // v1.0 使用配置中的 dp 值估算
            let width_dp = config.screen_width_dp().unwrap_or(360);
            let height_dp = config.screen_height_dp().unwrap_or(640);
            let density = config.density().unwrap_or(160) as f32 / 160.0;

            Ok(ScreenInfo {
                width: (width_dp as f32 * density) as u32,
                height: (height_dp as f32 * density) as u32,
                dpi: config.density().unwrap_or(160) as f32,
                scale_factor: density,
                orientation,
            })
        }
        #[cfg(not(target_os = "android"))]
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
