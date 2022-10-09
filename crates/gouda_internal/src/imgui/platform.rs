use std::time::Duration;

use imgui::{BackendFlags, Context, Key};

use crate::window::GameWindow;

pub struct GoudaImguiPlatform {}

impl GoudaImguiPlatform {
    pub fn init(imgui: &mut Context) {
        let io = imgui.io_mut();
        io.backend_flags.insert(BackendFlags::HAS_MOUSE_CURSORS);
        io.backend_flags.insert(BackendFlags::HAS_SET_MOUSE_POS);

        io[Key::Tab] = 0;
        io[Key::LeftArrow] = 1;
        io[Key::RightArrow] = 2;
        io[Key::UpArrow] = 3;
        io[Key::DownArrow] = 4;
        io[Key::PageUp] = 5;
        io[Key::PageDown] = 6;
        io[Key::Home] = 7;
        io[Key::End] = 8;
        io[Key::Insert] = 9;
        io[Key::Delete] = 10;
        io[Key::Backspace] = 11;
        io[Key::Space] = 12;
        io[Key::Enter] = 13;
        io[Key::Escape] = 14;
        io[Key::KeyPadEnter] = 15;
        io[Key::A] = 16;
        io[Key::C] = 17;
        io[Key::V] = 18;
        io[Key::X] = 19;
        io[Key::Y] = 20;
        io[Key::Z] = 21;

        imgui.set_platform_name(Some("gouda-imgui-platform".to_string()));
    }

    pub fn prepare_frame(context: &mut Context, window: &GameWindow, dt: Duration) {
        let io = context.io_mut();
        io.update_delta_time(dt);

        io.display_size = [window.get_width() as f32, window.get_height() as f32];
        io.display_framebuffer_scale = [1., 1.];
    }
}
