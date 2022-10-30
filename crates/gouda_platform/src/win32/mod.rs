#![cfg(target_os = "windows")]

use gouda_rendering::Renderer;

use std::rc::Rc;

use gouda_window::{GameWindow, PlatformWindow, WindowProps};

pub struct Win32PlatformLayer {
    window: GameWindow,
    renderer: Rc<Renderer>,
}

impl Win32PlatformLayer {
    pub fn new(props: WindowProps) -> Self {
        let mut window = PlatformWindow::new(props);
        let hwnd = window.hwnd;
        let renderer = Renderer::new(&mut window);
        let game_window = GameWindow::new(window);
        println!("building renderer");
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
