use gouda_input::GameInput;

#[cfg(target_os = "macos")]
pub mod osx;

#[cfg(target_os = "macos")]
pub use osx::PlatformWindow;

#[cfg(target_os = "windows")]
pub mod win32;

#[cfg(target_os = "windows")]
pub use win32::PlatformWindow;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

pub struct GameWindow {
    pub platform_window: PlatformWindow,
}

impl GameWindow {
    pub fn new(platform_window: PlatformWindow) -> Self {
        Self { platform_window }
    }

    pub fn capture_events(&mut self) -> Vec<WindowEvent> {
        self.platform_window.capture_events()
    }

    pub fn capture_input(&mut self) -> GameInput {
        self.platform_window.capture_input()
    }

    pub fn get_width(&self) -> usize {
        self.platform_window.get_width()
    }

    pub fn get_height(&self) -> usize {
        self.platform_window.get_height()
    }
}

pub trait GameWindowImpl {
    fn capture_events(&mut self) -> Vec<WindowEvent>;
    fn capture_input(&mut self) -> GameInput;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

pub struct WindowProps {
    pub width: f64,
    pub height: f64,
    pub title: String,
    pub target_ms_per_frame: f32,
}

pub enum WindowEvent {
    ResizeEvent { width: f32, height: f32 },
    CloseEvent,
}
