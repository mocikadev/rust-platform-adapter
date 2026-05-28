use crate::error::Result;
use crate::traits::ScreenProvider;
use crate::types::{Orientation, ScreenInfo};

use super::ffi::*;

pub struct OhosScreenProvider;

impl ScreenProvider for OhosScreenProvider {
    fn screen_info(&self) -> Result<ScreenInfo> {
        // OpenHarmony NDK C API (oh_display_manager.h)
        #[cfg(target_os = "ohos")]
        {
            let mut width: i32 = 0;
            let mut height: i32 = 0;
            let mut dpi: i32 = 0;
            let mut vpr: f32 = 1.0;
            let mut orientation: i32 = 0;

            // Safety: OH_NativeDisplayManager_* 系列函数是 OpenHarmony DisplayManager NDK 提供的线程安全函数，
            // 所有输出参数均为栈上合法变量的可变引用，函数仅写入对应类型的值，
            // 不会越界访问或修改其他内存
            unsafe {
                let r1 = OH_NativeDisplayManager_GetDefaultDisplayWidth(&mut width);
                let r2 = OH_NativeDisplayManager_GetDefaultDisplayHeight(&mut height);
                let r3 = OH_NativeDisplayManager_GetDefaultDisplayDensityDpi(&mut dpi);
                let _ = OH_NativeDisplayManager_GetDefaultDisplayVirtualPixelRatio(&mut vpr);
                let _ = OH_NativeDisplayManager_GetDefaultDisplayOrientation(&mut orientation);

                if r1 == DISPLAY_MANAGER_OK
                    && r2 == DISPLAY_MANAGER_OK
                    && r3 == DISPLAY_MANAGER_OK
                    && width > 0
                    && height > 0
                {
                    let orientation = match orientation {
                        DISPLAY_MANAGER_PORTRAIT => Orientation::Portrait,
                        DISPLAY_MANAGER_LANDSCAPE => Orientation::Landscape,
                        _ => Orientation::Unknown,
                    };

                    return Ok(ScreenInfo {
                        width: width as u32,
                        height: height as u32,
                        dpi: dpi as f32,
                        scale_factor: vpr,
                        orientation,
                    });
                }
            }
            Err(crate::error::PlatformError::FfiError(
                "Failed to get display info".to_string(),
            ))
        }
        #[cfg(not(target_os = "ohos"))]
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
