use crate::error::{PlatformError, Result};
use crate::traits::ScreenProvider;
use crate::types::{Orientation, ScreenInfo};

pub struct LinuxScreenProvider;

impl ScreenProvider for LinuxScreenProvider {
    fn screen_info(&self) -> Result<ScreenInfo> {
        #[cfg(target_os = "linux")]
        {
            // 优先尝试 X11 (x11rb + RandR)
            #[cfg(feature = "x11")]
            {
                if std::env::var("DISPLAY").is_ok() {
                    return Self::screen_info_x11();
                }
            }

            // Wayland: 当前不直接支持屏幕信息获取
            // Wayland 安全模型要求通过 xdg-desktop-portal 交互，过于重量级
            if std::env::var("WAYLAND_DISPLAY").is_ok() {
                return Err(PlatformError::NotSupported);
            }

            // 无显示服务器
            Err(PlatformError::NotSupported)
        }
        #[cfg(not(target_os = "linux"))]
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

#[cfg(all(target_os = "linux", feature = "x11"))]
impl LinuxScreenProvider {
    fn screen_info_x11() -> Result<ScreenInfo> {
        use x11rb::connection::Connection;
        use x11rb::protocol::randr::{
            Connection as RandrConnection, ConnectionExt as RandrConnectionExt,
        };
        use x11rb::protocol::xproto::ConnectionExt;

        let (conn, screen_num) = x11rb::connect(None)
            .map_err(|e| PlatformError::FfiError(format!("X11 connect failed: {}", e)))?;

        let setup = conn.setup();
        let screen = &setup.roots[screen_num];
        let root = screen.root;

        // 获取屏幕尺寸（像素）
        let geom = conn
            .get_geometry(root)
            .map_err(|e| PlatformError::FfiError(format!("GetGeometry request failed: {}", e)))?
            .reply()
            .map_err(|e| PlatformError::FfiError(format!("GetGeometry reply failed: {}", e)))?;

        let width = geom.width as u32;
        let height = geom.height as u32;

        // 尝试通过 RandR 获取 DPI 和缩放因子
        let (dpi, scale_factor) = match conn.randr_get_screen_resources_current(root) {
            Ok(cookie) => match cookie.reply() {
                Ok(resources) => {
                    let mut found = None;
                    for output in &resources.outputs {
                        let info_cookie = match conn.randr_get_output_info(*output, 0) {
                            Ok(c) => c,
                            Err(_) => continue,
                        };
                        let info = match info_cookie.reply() {
                            Ok(i) => i,
                            Err(_) => continue,
                        };
                        {
                            if info.connection == RandrConnection::CONNECTED {
                                let w_mm = info.mm_width as f32;
                                let h_mm = info.mm_height as f32;
                                if w_mm > 0.0 && h_mm > 0.0 {
                                    let dpi_x = geom.width as f32 / (w_mm / 25.4);
                                    let dpi_y = geom.height as f32 / (h_mm / 25.4);
                                    let dpi = (dpi_x + dpi_y) / 2.0;
                                    let scale = (dpi / 96.0).max(1.0);
                                    found = Some((dpi, scale));
                                    break;
                                }
                            }
                        }
                    }
                    found.unwrap_or((96.0, 1.0))
                }
                Err(_) => (96.0, 1.0),
            },
            Err(_) => (96.0, 1.0),
        };

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
