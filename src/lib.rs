use crate::ecs::ECS;
use crate::input::GameInput;
use crate::platform::PlatformLayer;
use crate::window::WindowProps;
use std::thread::sleep;
use std::time;
use std::time::Instant;

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
            width: 500.,
            height: 600.,
            title: "Gouda Test".to_string(),
            target_ms_per_frame: 30.,
        };
        let mut platform = PlatformLayer::new(props);

        let mut now = Instant::now();
        loop {
            let mut window = platform.get_window();
            let input = window.capture_input();
            self.update(input);

            let renderer = platform.get_renderer();
            renderer.render();


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
