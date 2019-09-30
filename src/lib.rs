use crate::ecs::ECS;
use crate::input::GameInput;
use crate::platform::PlatformLayer;
use crate::window::WindowProps;
use std::thread::sleep;
use std::time;
use std::time::Instant;
use crate::platform::metal::{SquareDrawable, TriangleDrawable, Drawable};

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[macro_use]
extern crate enum_map;

pub mod ecs;
mod input;
mod platform;
mod window;
mod rendering;
mod math;

pub trait GameLogic {
    fn register_components(&self, ecs: &mut ECS);
    fn register_systems(&self, ecs: &mut ECS);
    fn setup(&self, ecs: &mut ECS);
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
        self.setup_game();

        let props = WindowProps {
            width: 600.,
            height: 600.,
            title: "Gouda Test".to_string(),
            target_ms_per_frame: 30.,
        };
        let mut platform = PlatformLayer::new(props);

        let mut now = Instant::now();

        let renderer = platform.get_renderer();
        let mut square = SquareDrawable::new(renderer, [0.1, 0.2, 0.], 0.3, [0., 0.3, 1.]);
        let mut square2 = SquareDrawable::new(renderer, [0., 0.5, 0.], 0.2, [-0.5, 0.0, 0.]);
        let mut square3 = SquareDrawable::new(renderer, [0., 0., 0.5], 0.4, [0.5, -0.3, 0.]);
        let mut tri = TriangleDrawable::new(renderer);

        loop {
            let window = platform.get_window();
            let input = window.capture_input();
            self.update(input.clone());

            let renderer = platform.get_renderer();
            if let Some(scene) = renderer.begin_scene() {
                square.update(&input);
                square.bind(&scene);
                square.draw(&scene);

                square2.update(&input);
                square2.bind(&scene);
                square2.draw(&scene);

                square3.update(&input);
                square3.bind(&scene);
                square3.draw(&scene);

//                tri.bind(&scene);
//                tri.draw(&scene);
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
