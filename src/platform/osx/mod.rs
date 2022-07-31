#![cfg(target_os = "macos")]

use crate::window::{GameWindow, WindowProps};
use cocoa::foundation::NSAutoreleasePool;
use cocoa::base::{nil, id};
use cocoa::appkit::{NSApp, NSApplication};
use cocoa::appkit::{NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
                    NSApplicationPresentationOptions};
use objc::runtime::YES;
use crate::rendering::Renderer;
use crate::platform::osx::osx_window::OsxWindow;
use std::rc::Rc;

pub mod osx_window;
pub mod osx_input;

#[allow(dead_code)]
pub struct OSXPlatformLayer {
    pool: id,
    window: GameWindow,
    renderer: Rc<Renderer>,
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
        let renderer = Renderer::new(&mut window);
        window.attach_renderer(&renderer);

        OSXPlatformLayer {
            pool,
            window: GameWindow::new(Box::new(window)),
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



