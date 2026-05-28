use crate::error::Result;
use crate::traits::ScreenProvider;
use crate::types::{Orientation, ScreenInfo};

pub struct OhosScreenProvider;

impl ScreenProvider for OhosScreenProvider {
    fn screen_info(&self) -> Result<ScreenInfo> {
        // OpenHarmony NDK C API (oh_display_manager.h)
        #[cfg(target_os = "ohos")]
        {
            type NativeDisplayManager_ErrorCode = i32;
            const DISPLAY_MANAGER_OK: i32 = 0;

            // NativeDisplayManager_Orientation 枚举
            const DISPLAY_MANAGER_PORTRAIT: i32 = 0;
            const DISPLAY_MANAGER_LANDSCAPE: i32 = 1;

            extern "C" {
                fn OH_NativeDisplayManager_GetDefaultDisplayWidth(
                    width: *mut i32,
                ) -> NativeDisplayManager_ErrorCode;
                fn OH_NativeDisplayManager_GetDefaultDisplayHeight(
                    height: *mut i32,
                ) -> NativeDisplayManager_ErrorCode;
                fn OH_NativeDisplayManager_GetDefaultDisplayDensityDpi(
                    dpi: *mut i32,
                ) -> NativeDisplayManager_ErrorCode;
                fn OH_NativeDisplayManager_GetDefaultDisplayVirtualPixelRatio(
                    vpr: *mut f32,
                ) -> NativeDisplayManager_ErrorCode;
                fn OH_NativeDisplayManager_GetDefaultDisplayOrientation(
                    orientation: *mut i32,
                ) -> NativeDisplayManager_ErrorCode;
            }

            let mut width: i32 = 0;
            let mut height: i32 = 0;
            let mut dpi: i32 = 0;
            let mut vpr: f32 = 1.0;
            let mut orientation: i32 = 0;

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
