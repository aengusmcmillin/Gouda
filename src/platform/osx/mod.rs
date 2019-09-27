#![cfg(target_os = "macos")]

use crate::platform::PlatformLayerImpl;
use crate::window::{GameWindow, WindowProps};
use cocoa::foundation::NSAutoreleasePool;
use cocoa::base::{nil, id};
use cocoa::appkit::{NSApp, NSApplication};
use cocoa::appkit::{NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
                    NSApplicationPresentationOptions};
use objc::runtime::YES;

pub mod cocoa_window;
pub mod osx_input;

pub struct OSXPlatformLayer {
    pool: id,
}

impl OSXPlatformLayer {
    pub fn new() -> Self {
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

        OSXPlatformLayer {
            pool,
        }
    }
}
impl PlatformLayerImpl for OSXPlatformLayer {
    fn create_window(&self, props: WindowProps) -> GameWindow {
        GameWindow::new(props)
    }
}



