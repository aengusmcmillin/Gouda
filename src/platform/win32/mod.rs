#![cfg(target_os = "windows")]

use crate::window::{GameWindow, WindowProps};
use crate::platform::d3d::Renderer;
use std::rc::Rc;

pub struct Win32PlatformLayer {
}

impl Win32PlatformLayer {
    pub fn new(props: WindowProps) -> Self {
        Self {}
    }

    pub fn get_window(&mut self) -> &mut GameWindow {
        unimplemented!();
    }

    pub fn get_renderer(&mut self) -> &Rc<Renderer> {
        unimplemented!();
    }
}