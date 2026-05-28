use crate::error::Result;
use crate::traits::ScreenProvider;
use crate::types::{Orientation, ScreenInfo};

pub struct WindowsScreenProvider;

impl ScreenProvider for WindowsScreenProvider {
    fn screen_info(&self) -> Result<ScreenInfo> {
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::Graphics::Gdi::{GetDC, GetDeviceCaps, LOGPIXELSX};
            use windows::Win32::UI::WindowsAndMessaging::{
                GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN,
            };

            unsafe {
                let width = GetSystemMetrics(SM_CXSCREEN) as u32;
                let height = GetSystemMetrics(SM_CYSCREEN) as u32;

                let hdc = GetDC(None);
                let dpi = if hdc.is_invalid() {
                    96.0 // 默认 DPI
                } else {
                    GetDeviceCaps(hdc, LOGPIXELSX) as f32
                };

                let scale_factor = dpi / 96.0;

                let orientation = if width >= height {
                    Orientation::Landscape
                } else {
                    Orientation::Portrait
                };

                Ok(ScreenInfo {
                    width,
                    height,
                    dpi,
                    scale_factor,
                    orientation,
                })
            }
        }
        #[cfg(not(target_os = "windows"))]
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
