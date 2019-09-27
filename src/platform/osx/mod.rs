#![cfg(target_os = "macos")]

use crate::platform::PlatformLayerImpl;
use crate::window::{GameWindow, WindowProps};
use cocoa::foundation::NSAutoreleasePool;
use cocoa::base::{nil, id};
use cocoa::appkit::{NSApp, NSApplication};
use cocoa::appkit::{NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
                    NSApplicationPresentationOptions};
use objc::runtime::YES;
use crate::platform::metal::MetalRenderer;
use crate::rendering::Renderer;
use crate::platform::osx::cocoa_window::OsxWindow;

pub mod cocoa_window;
pub mod osx_input;

pub struct OSXPlatformLayer {
    pool: id,
    window: GameWindow,
    renderer: Renderer,
}

impl OSXPlatformLayer {
    pub fn new(props: WindowProps) -> Self {
        let pool = unsafe {NSAutoreleasePool::new(nil)};
        unsafe {
            let application = NSApp();
            application.setActivationPolicy_(NSApplicationActivationPolicyRegular);
            application.setPresentationOptions_(
                NSApplicationPresentationOptions::NSApplicationPresentationDefault,
            );

            NSApplication::finishLaunching(nil);
            NSApp().activateIgnoringOtherApps_(YES);
        }

        let mut window = OsxWindow::new(props);
        let renderer = MetalRenderer::new(&mut window);
        window.attach_renderer(&renderer);

        OSXPlatformLayer {
            pool,
            window: GameWindow::new(Box::new(window)),
            renderer: Renderer::new(Box::new(renderer)),
        }
    }
}

impl PlatformLayerImpl for OSXPlatformLayer {
    fn get_window(&mut self) -> &mut GameWindow {
        return &mut self.window;
    }

    fn get_renderer(&mut self) -> &mut Renderer {
        return &mut self.renderer;
    }
}



