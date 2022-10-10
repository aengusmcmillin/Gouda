#![cfg(target_os = "windows")]

pub mod win32_input;
pub mod window;

use gouda_rendering::Renderer;

use std::rc::Rc;

use crate::win32::window::Window;
use crate::window::{GameWindow, WindowProps};

pub struct Win32PlatformLayer {
    window: GameWindow,
    renderer: Rc<Renderer>,
}

impl Win32PlatformLayer {
    pub fn new(props: WindowProps) -> Self {
        let window = Window::new(props);
        let hwnd = window.hwnd;
        let game_window = GameWindow::new(Box::new(window));
        println!("building renderer");
        let renderer = Renderer::new(hwnd);
        println!("renderer built");
        Self {
            window: game_window,
            renderer: Rc::new(renderer.unwrap()),
        }
    }

    pub fn get_window(&mut self) -> &mut GameWindow {
        return &mut self.window;
    }

    pub fn get_renderer(&mut self) -> &Rc<Renderer> {
        return &self.renderer;
    }
}
