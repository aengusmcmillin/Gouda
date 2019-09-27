use crate::window::{WindowProps, GameWindowImpl};

extern crate libc;

extern crate coreaudio_sys;

use cocoa::{
    appkit::{
        NSApp, NSApplication,
        NSBackingStoreBuffered, NSEvent, NSEventMask,
        NSEventModifierFlags, NSEventType, NSMenu, NSMenuItem, NSWindow, NSWindowStyleMask,
    },
    base::{id, nil, selector, NO},
    foundation::{
        NSAutoreleasePool, NSDate, NSDefaultRunLoopMode, NSPoint, NSRect, NSSize,
        NSString,
    },
};

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, YES};

use crate::input::GameInput;
use cocoa::appkit::NSView;
use crate::platform::osx::osx_input::{osx_process_key, osx_process_keyboard_message};
use std::mem;
use crate::platform::metal::MetalRenderer;

pub struct OsxWindow {
    cocoa_window: CocoaWindow,
    props: WindowProps,
    input: GameInput,
}

impl OsxWindow {
    pub fn new(props: WindowProps) -> Self {
        create_menu_bar(&props.title);
        let frame_rect = NSRect {
            origin: NSPoint { x: 0., y: 0. },
            size: NSSize {
                width: props.width,
                height: props.height,
            },
        };

        let mut cocoa_window = show_window(&props.title, props.width, props.height);
        cocoa_window.create_view(frame_rect);

        let mut input = GameInput::default();
        input.seconds_to_advance_over_update = props.target_ms_per_frame / 1000.;

        OsxWindow {
            cocoa_window,
            props,
            input,
        }
    }

    pub fn attach_renderer(&self, renderer: &MetalRenderer) {
        self.cocoa_window.attach_renderer(renderer);
    }
}

fn create_menu_bar(title: &String) {
    unsafe {
        let app_name = NSString::alloc(nil).init_str(title);
        let menu_bar = NSMenu::new(nil).autorelease();
        let app_menu_item = NSMenuItem::new(nil).autorelease();
        menu_bar.addItem_(app_menu_item);

        NSApp().setMainMenu_(menu_bar);

        let app_menu = NSMenu::new(nil).autorelease();
        let quit_prefix = NSString::alloc(nil).init_str("Quit ");
        let quit_title = quit_prefix.stringByAppendingString_(app_name);
        let quit_action = selector("terminate:");
        let quit_key = NSString::alloc(nil).init_str("q");
        let quit_menu_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(quit_title, quit_action, quit_key)
            .autorelease();
        app_menu.addItem_(quit_menu_item);
        app_menu_item.setSubmenu_(app_menu);
    }
}

fn show_window(title: &String, width: f64, height: f64) -> CocoaWindow {
    unsafe {
        let app_name = NSString::alloc(nil).init_str(title);
        let frame = NSRect::new(NSPoint::new(0., 0.), NSSize::new(width, height));
        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(
                frame,
                NSWindowStyleMask::NSTitledWindowMask
                    | NSWindowStyleMask::NSResizableWindowMask
                    | NSWindowStyleMask::NSClosableWindowMask,
                NSBackingStoreBuffered,
                NO,
            )
            .autorelease();
        window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
        window.setTitle_(app_name);
        window.makeKeyAndOrderFront_(nil);

        let superclass = Class::get("NSObject").unwrap();
        let mut decl = ClassDecl::new("WindowDelegate", superclass).unwrap();
        extern "C" fn window_will_close(_: &Object, _: Sel, _: id) {
            unsafe {
            }
        }

        decl.add_method(
            sel!(windowWillClose:),
            window_will_close as extern "C" fn(&Object, Sel, id),
        );
        let delegate_class = decl.register();
        let delegate_object = msg_send![delegate_class, new];
        window.setDelegate_(delegate_object);

        let ns_view_class = Class::get("NSView").unwrap();
        let mut view_decl = ClassDecl::new("FlippedCoordNSView", ns_view_class).unwrap();
        extern "C" fn is_flipped(_: &Object, _: Sel) -> bool {
            return true;
        }
        view_decl.add_method(
            sel!(isFlipped),
            is_flipped as extern "C" fn(&Object, Sel) -> bool,
        );
        let view_class = view_decl.register();
        let view_object = msg_send![view_class, new];
        window.setContentView_(view_object);

        return CocoaWindow::new(window);
    }
}

pub struct CocoaWindow {
    window: id,
    view: Option<id>,
}

impl CocoaWindow {
    pub fn new(window: id) -> CocoaWindow {
        CocoaWindow{window, view: None}
    }

    pub fn create_view(&mut self, frame_rect: NSRect) {
        unsafe {
            let view = NSView::alloc(nil).initWithFrame_(frame_rect);
            view.setWantsBestResolutionOpenGLSurface_(YES);
            view.setWantsLayer(YES);
            self.window.contentView().addSubview_(view);

            self.view = Some(view);
        }
    }

    pub fn mouseLocationOutsideOfEventStream(&self) -> NSPoint {
        unsafe {
            let loc = self.window.mouseLocationOutsideOfEventStream();
            return loc;
        }
    }

    pub fn attach_renderer(&self, renderer: &MetalRenderer) {
        unsafe {
            self.view.unwrap().setLayer(mem::transmute(renderer.get_layer()));
        }
    }
}

impl GameWindowImpl for OsxWindow {
    fn capture_input(&mut self) -> GameInput {
        unsafe {
            self.input = GameInput::from(&self.input);

            let pool2 = NSAutoreleasePool::new(nil);
            let mut event = NSApp().nextEventMatchingMask_untilDate_inMode_dequeue_(
                NSEventMask::NSAnyEventMask.bits(),
                NSDate::distantPast(nil),
                NSDefaultRunLoopMode,
                YES,
            );
            while !(event == nil) {
                match event.eventType() {
                    NSEventType::NSKeyDown | NSEventType::NSKeyUp => {
                        let key_down = event.eventType() == NSEventType::NSKeyDown;

                        let mut keyboard = &mut self.input.keyboard;
                        let chars = event.charactersIgnoringModifiers();
                        let bytes = chars.UTF8String() as *const u8;
                        let objc_string =
                            std::str::from_utf8(std::slice::from_raw_parts(bytes, chars.len()))
                                .unwrap();
                        let u16_char = objc_string.encode_utf16().next().unwrap();
                        let modifiers = event.modifierFlags();
                        let is_cmd = modifiers.contains(NSEventModifierFlags::NSCommandKeyMask);
                        let is_alternate =
                            modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask);
                        let is_control =
                            modifiers.contains(NSEventModifierFlags::NSControlKeyMask);

                        osx_process_key(
                            &mut keyboard,
                            u16_char,
                            key_down,
                            is_alternate,
                            is_control,
                            is_cmd,
                        );
                    }
                    NSEventType::NSMouseMoved => {
                        let location = self.cocoa_window.mouseLocationOutsideOfEventStream();
                        self.input.mouse.x = location.x as i32;
                        self.input.mouse.y = self.props.height as i32 - location.y as i32;
                        NSApp().sendEvent_(event);
                    }
                    NSEventType::NSLeftMouseUp | NSEventType::NSLeftMouseDown => {
                        let is_down = event.eventType() == NSEventType::NSLeftMouseDown;
                        osx_process_keyboard_message(&mut self.input.mouse.buttons[0], is_down);
                        NSApp().sendEvent_(event);
                    }
                    NSEventType::NSRightMouseUp | NSEventType::NSRightMouseDown => {
                        let is_down = event.eventType() == NSEventType::NSRightMouseDown;
                        osx_process_keyboard_message(&mut self.input.mouse.buttons[2], is_down);
                        NSApp().sendEvent_(event);
                    }
                    _ => {
                        NSApp().sendEvent_(event);
                    }
                }

                event = NSApp().nextEventMatchingMask_untilDate_inMode_dequeue_(
                    NSEventMask::NSAnyEventMask.bits(),
                    NSDate::distantPast(nil),
                    NSDefaultRunLoopMode,
                    YES,
                );
            }
            msg_send![pool2, release];
            return self.input.clone();
        }
    }

    fn get_width(&self) -> usize {
        self.props.width as usize
    }

    fn get_height(&self) -> usize {
        self.props.height as usize
    }
}

