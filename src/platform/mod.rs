use crate::platform::osx::OSXPlatformLayer;
use crate::window::{GameWindow, WindowProps};

#[cfg(target_os = "macos")]
pub mod osx;

pub struct PlatformLayer {
    platform_impl: Box<dyn PlatformLayerImpl>,
}

impl PlatformLayer {
    pub fn new() -> Self {
        PlatformLayer {
            platform_impl: Box::new(OSXPlatformLayer::new()),
        }
    }

    pub fn create_window(&self, props: WindowProps) -> GameWindow {
        self.platform_impl.create_window(props)
    }
}

pub trait PlatformLayerImpl {
    fn create_window(&self, props: WindowProps) -> GameWindow;
}

