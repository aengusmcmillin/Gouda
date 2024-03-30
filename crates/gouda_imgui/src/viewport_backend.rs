use std::{collections::HashMap, ffi::c_void, hash::Hash, iter::Map};

use gouda_rendering::Renderer;
use gouda_window::{GameWindow, PlatformWindow, WindowProps};
use imgui::{Id, PlatformViewportBackend};


pub struct GoudaViewportBackend {
    windows: HashMap<Id, PlatformWindow>,
    renderers: HashMap<Id, Renderer>,
}

impl GoudaViewportBackend {
    pub fn new() -> Self {
        Self { 
            windows: HashMap::new(),
            renderers: HashMap::new(),
        }
    }
}

impl PlatformViewportBackend for GoudaViewportBackend {
    fn create_window(&mut self, viewport: &mut imgui::Viewport) {
        let props = WindowProps {
            width: viewport.size[0] as f64,
            height: viewport.size[1] as f64,
            title: "BLAH".to_owned(),
            target_ms_per_frame: 16.0
        };
        let mut window = PlatformWindow::new(props);
        let mut renderer = Renderer::new(&mut window).unwrap();
        viewport.platform_user_data = &mut window as *mut _ as *mut c_void;
        viewport.renderer_user_data = &mut renderer as *mut _ as *mut c_void;
    }

    fn destroy_window(&mut self, viewport: &mut imgui::Viewport) {
        todo!()
    }

    fn show_window(&mut self, viewport: &mut imgui::Viewport) {
        todo!()
    }

    fn set_window_pos(&mut self, viewport: &mut imgui::Viewport, pos: [f32; 2]) {
        todo!()
    }

    fn get_window_pos(&mut self, viewport: &mut imgui::Viewport) -> [f32; 2] {
        todo!()
    }

    fn set_window_size(&mut self, viewport: &mut imgui::Viewport, size: [f32; 2]) {
        todo!()
    }

    fn get_window_size(&mut self, viewport: &mut imgui::Viewport) -> [f32; 2] {
        todo!()
    }

    fn set_window_focus(&mut self, viewport: &mut imgui::Viewport) {
        todo!()
    }

    fn get_window_focus(&mut self, viewport: &mut imgui::Viewport) -> bool {
        todo!()
    }

    fn get_window_minimized(&mut self, viewport: &mut imgui::Viewport) -> bool {
        todo!()
    }

    fn set_window_title(&mut self, viewport: &mut imgui::Viewport, title: &str) {
        todo!()
    }

    fn set_window_alpha(&mut self, viewport: &mut imgui::Viewport, alpha: f32) {
        todo!()
    }

    fn update_window(&mut self, viewport: &mut imgui::Viewport) {
        todo!()
    }

    fn render_window(&mut self, viewport: &mut imgui::Viewport) {
        let renderer: &mut Renderer = unsafe { &mut *(viewport.renderer_user_data as *mut Renderer) };

        let mut scene = renderer.begin_scene().unwrap();
        scene.submit_imgui(vbuf, ibuf, count, vtx_offset, idx_offset, texture, matrix)
        viewport.draw_data()
        todo!()
    }

    fn swap_buffers(&mut self, viewport: &mut imgui::Viewport) {
        todo!()
    }

    fn create_vk_surface(
        &mut self,
        viewport: &mut imgui::Viewport,
        instance: u64,
        out_surface: &mut u64,
    ) -> i32 {
        todo!()
    }
}