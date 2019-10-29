use crate::ecs::ECS;
use crate::input::{GameInput, LetterKeys};
use crate::platform::PlatformLayer;
use crate::window::WindowProps;
use std::thread::sleep;
use std::time;
use std::time::Instant;
use crate::rendering::drawable::{TriangleDrawable, Drawable, QuadDrawable};
use crate::rendering::Scene;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[macro_use]
extern crate enum_map;

pub mod ecs;
pub mod input;
mod platform;
mod window;
pub mod rendering;
pub mod math;
mod utils;
pub mod bmp;
pub mod png;
pub mod font;

pub trait GameLogic {
    fn register_components(&self, ecs: &mut ECS);
    fn register_systems(&self, ecs: &mut ECS);
    fn setup(&mut self, ecs: &mut ECS);
    fn draw_scene(&self, ecs: &ECS, scene: &Scene);
}

pub struct Gouda<T: GameLogic> {
    game_logic: T,
    ecs: ECS,
}

impl<T: GameLogic> Gouda<T> {
    pub fn new(game_logic: T) -> Self {
        Gouda {
            game_logic,
            ecs: ECS::new(),
        }
    }

    fn setup_engine(&mut self) {
        self.ecs.add_res(GameInput::new());
    }

    fn setup_game(&mut self) {
        self.game_logic.register_components(&mut self.ecs);
        self.game_logic.register_systems(&mut self.ecs);

        self.game_logic.setup(&mut self.ecs);
    }

    fn update(&mut self, game_input: GameInput) {
        (*self.ecs.write_res::<GameInput>()) = game_input;

        self.ecs.run_systems();
    }

    pub fn run(&mut self) {
        self.setup_engine();

        let props = WindowProps {
            width: 900.,
            height: 900.,
            title: "Gouda Test".to_string(),
            target_ms_per_frame: 30.,
        };
        let mut platform = PlatformLayer::new(props);

        let mut now = Instant::now();

        let renderer = platform.get_renderer();

        self.ecs.add_res(renderer.clone());

        self.setup_game();

        loop {
            let window = platform.get_window();
            let input = window.capture_input();
            self.update(input.clone());

            let renderer = platform.get_renderer();
            if let Some(scene) = renderer.begin_scene() {
                self.game_logic.draw_scene(&self.ecs, &scene);
                renderer.end_scene(scene);
            }

            let next = Instant::now();
            let delta = next - now;
            let target_dur = time::Duration::from_millis(30);
            if target_dur > delta {
                let wait = target_dur - delta;
                sleep(wait);
            }
            let next = Instant::now();
            now = next;
        }
    }
}
