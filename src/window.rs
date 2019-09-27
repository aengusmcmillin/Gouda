use crate::input::GameInput;
use crate::platform::osx::cocoa_window::OsxWindow;

pub struct GameWindow {
    game_window_impl: Box<dyn GameWindowImpl>,
}


impl GameWindow {
    pub fn new(props: WindowProps) -> Self {
        Self {
            game_window_impl: Box::new(OsxWindow::new(props)),
        }
    }

    pub fn capture_input(&mut self) -> GameInput {
        self.game_window_impl.capture_input()
    }
}

pub trait GameWindowImpl {
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