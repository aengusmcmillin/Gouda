use crate::input::GameInput;

pub struct GameWindow {
    game_window_impl: Box<dyn GameWindowImpl>,
}


impl GameWindow {
    pub fn new(platform_impl: Box<dyn GameWindowImpl>) -> Self {
        Self {
            game_window_impl: platform_impl,
        }
    }

    pub fn capture_events(&mut self) -> Vec<WindowEvent> {
        self.game_window_impl.capture_events()
    }

    pub fn capture_input(&mut self) -> GameInput {
        self.game_window_impl.capture_input()
    }

    pub fn get_width(&self) -> usize {
        self.game_window_impl.get_width()
    }

    pub fn get_height(&self) -> usize {
        self.game_window_impl.get_height()
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
    ResizeEvent {width: f32, height: f32},
    CloseEvent,
}