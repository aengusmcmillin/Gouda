#![cfg(target_os = "windows")]

pub mod window;

use crate::window::{GameWindow, WindowProps};
use crate::platform::d3d::Renderer;
use std::rc::Rc;
use crate::platform::win32::window::Window;

pub struct Win32PlatformLayer {
    window: GameWindow,
    renderer: Rc<Renderer>,
}

impl Win32PlatformLayer {
    pub fn new(props: WindowProps) -> Self {
        Self {
            window: GameWindow::new(Box::new(Window::new(props))),
            renderer: Rc::new(Renderer{}),
        }
    }

    pub fn get_window(&mut self) -> &mut GameWindow {
        return &mut self.window;
    }

    pub fn get_renderer(&mut self) -> &Rc<Renderer> {
        return &self.renderer;
    }
}