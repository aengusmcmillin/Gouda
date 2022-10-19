#![cfg(target_os = "macos")]

use cocoa::appkit::NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular;
use cocoa::appkit::{NSApp, NSApplication, NSApplicationPresentationOptions};
use cocoa::base::{id, nil};
use cocoa::foundation::NSAutoreleasePool;
use gouda_rendering::Renderer;
use gouda_window::osx::PlatformWindow;
use gouda_window::{GameWindow, WindowProps};
use objc::runtime::YES;
use std::rc::Rc;

#[allow(dead_code)]
pub struct OSXPlatformLayer {
    pool: id,
    window: GameWindow,
    renderer: Rc<Renderer>,
}

impl OSXPlatformLayer {
    pub fn new(props: WindowProps) -> Self {
        let pool = unsafe { NSAutoreleasePool::new(nil) };
        unsafe {
            let application = NSApp();
            application.setActivationPolicy_(NSApplicationActivationPolicyRegular);
            application.setPresentationOptions_(
                NSApplicationPresentationOptions::NSApplicationPresentationDefault,
            );

            NSApplication::finishLaunching(nil);
            NSApp().activateIgnoringOtherApps_(YES);
        }

        let mut window = PlatformWindow::new(props);
        let renderer = Renderer::new(&mut window).unwrap();
        renderer.platform_renderer.attach_renderer(&window);

        OSXPlatformLayer {
            pool,
            window: GameWindow::new(window),
            renderer: Rc::new(renderer),
        }
    }
    pub fn get_window(&mut self) -> &mut GameWindow {
        return &mut self.window;
    }

    pub fn get_renderer(&mut self) -> &Rc<Renderer> {
        return &self.renderer;
    }

    pub fn get_mut_renderer(&mut self) -> &mut Renderer {
        Rc::get_mut(&mut self.renderer).unwrap()
    }
}
