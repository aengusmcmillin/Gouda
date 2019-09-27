use crate::platform::osx::OSXPlatformLayer;
use crate::window::{GameWindow, WindowProps};
use crate::rendering::Renderer;

#[cfg(target_os = "macos")]
pub mod osx;

#[cfg(target_os = "macos")]
pub mod metal;

pub struct PlatformLayer {
    platform_impl: Box<dyn PlatformLayerImpl>,
}

impl PlatformLayer {
    pub fn new(window_props: WindowProps) -> Self {
        PlatformLayer {
            platform_impl: Box::new(OSXPlatformLayer::new(window_props)),
        }
    }

    pub fn get_window(&mut self) -> &mut GameWindow {
        self.platform_impl.get_window()
    }

    pub fn get_renderer(&mut self) -> &mut Renderer {
        self.platform_impl.get_renderer()
    }
}

pub trait PlatformLayerImpl {
    fn get_window(&mut self) -> &mut GameWindow;
    fn get_renderer(&mut self) -> &mut Renderer;
}

