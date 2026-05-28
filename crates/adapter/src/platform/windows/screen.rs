use crate::error::{PlatformError, Result};
use crate::traits::ScreenProvider;
use crate::types::{Orientation, ScreenInfo};

pub struct WindowsScreenProvider;

impl ScreenProvider for WindowsScreenProvider {
    fn screen_info(&self) -> Result<ScreenInfo> {
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::Graphics::Gdi::{GetDC, GetDeviceCaps, ReleaseDC, LOGPIXELSX};
            use windows::Win32::UI::WindowsAndMessaging::{
                GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN,
            };

            unsafe {
                let width = GetSystemMetrics(SM_CXSCREEN);
                let height = GetSystemMetrics(SM_CYSCREEN);

                if width <= 0 || height <= 0 {
                    return Err(PlatformError::FfiError(
                        "GetSystemMetrics returned invalid screen dimensions".to_string(),
                    ));
                }

                let hdc = GetDC(None);
                let dpi = if hdc.is_invalid() {
                    96.0
                } else {
                    let dpi = GetDeviceCaps(Some(hdc), LOGPIXELSX) as f32;
                    ReleaseDC(None, hdc);
                    dpi
                };

                let scale_factor = dpi / 96.0;

                let orientation = if width as u32 >= height as u32 {
                    Orientation::Landscape
                } else {
                    Orientation::Portrait
                };

                Ok(ScreenInfo {
                    width: width as u32,
                    height: height as u32,
                    dpi,
                    scale_factor,
                    orientation,
                })
            }
        }
        #[cfg(not(target_os = "windows"))]
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
