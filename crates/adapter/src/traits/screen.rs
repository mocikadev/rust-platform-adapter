use std::future::Future;
use std::pin::Pin;

use crate::error::Result;
use crate::types::{Orientation, ScreenInfo};

/// 屏幕信息接口
pub trait ScreenProvider {
    /// 获取屏幕信息
    fn screen_info(&self) -> Result<ScreenInfo>;

    /// 获取屏幕宽度（像素）
    fn screen_width(&self) -> Result<u32>;

    /// 获取屏幕高度（像素）
    fn screen_height(&self) -> Result<u32>;

    /// 获取缩放因子
    fn scale_factor(&self) -> Result<f32>;

    /// 获取屏幕方向
    fn orientation(&self) -> Result<Orientation>;

    // ===== 异步接口 =====

    /// 异步获取屏幕信息
    fn screen_info_async(&self) -> Pin<Box<dyn Future<Output = Result<ScreenInfo>> + Send + '_>> {
        Box::pin(std::future::ready(self.screen_info()))
    }

    /// 异步获取屏幕方向
    fn orientation_async(&self) -> Pin<Box<dyn Future<Output = Result<Orientation>> + Send + '_>> {
        Box::pin(std::future::ready(self.orientation()))
    }
}
