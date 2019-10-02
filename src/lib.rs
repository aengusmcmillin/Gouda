use crate::ecs::ECS;
use crate::input::{GameInput, LetterKeys};
use crate::platform::PlatformLayer;
use crate::window::WindowProps;
use std::thread::sleep;
use std::time;
use std::time::Instant;
use crate::platform::metal::drawable::{CubeDrawable, TriangleDrawable, Drawable, QuadDrawable};
use crate::platform::metal::Scene;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[macro_use]
extern crate enum_map;

pub mod ecs;
mod input;
mod platform;
mod window;
pub mod rendering;
mod math;

pub trait GameLogic {
    fn register_components(&self, ecs: &mut ECS);
    fn register_systems(&self, ecs: &mut ECS);
    fn setup(&self, ecs: &mut ECS);
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

        self.setup_game();

        let mut quads = vec![];
        for x in 0..11 {
            for y in 0..9 {
                let color = if x == 5 && y == 4 {
                    [0.5, 0.2, 0.2]
                } else if x == 0 || x == 10 || y == 0 || y == 8 {
                    [0.5, 0.5, 0.5]
                } else {
                    [0.2, 0.4, 0.3]
                };
                quads.push(QuadDrawable::new(renderer, color, [-0.91 + x as f32 * 0.182, -0.52 + y as f32 * 0.179, 0.], [0.08, 0.08, 1.]));
            }
        }
        let bottom = QuadDrawable::new(renderer, [0.1, 0.1, 0.1], [-0., -0.80, 0.], [0.99, 0.19, 1.]);
        let player = QuadDrawable::new(renderer, [0.7, 0.7, 0.7], [-0.364, -0.165, 0.], [0.05, 0.05, 1.]);

        let mut player_x = -0.364;
        let mut player_y = -0.165;
        loop {
            let window = platform.get_window();
            let input = window.capture_input();
            self.update(input.clone());
            if input.keyboard.letter_pressed(LetterKeys::A) {
                player_x -= 0.182;
                player.translate([player_x, player_y, 0.], [0.05, 0.05, 1.]);
            } else if input.keyboard.letter_pressed(LetterKeys::D) {
                player_x += 0.182;
                player.translate([player_x, player_y, 0.], [0.05, 0.05, 1.]);
            }
            if input.keyboard.letter_pressed(LetterKeys::W) {
                player_y += 0.179;
                player.translate([player_x, player_y, 0.], [0.05, 0.05, 1.]);
            } else if input.keyboard.letter_pressed(LetterKeys::S) {
                player_y -= 0.179;
                player.translate([player_x, player_y, 0.], [0.05, 0.05, 1.]);
            }

            let renderer = platform.get_renderer();
            if let Some(scene) = renderer.begin_scene() {
                self.game_logic.draw_scene(&self.ecs, &scene);
                for quad in quads.iter() {
                    quad.draw(&scene);
                }
                bottom.draw(&scene);
                player.draw(&scene);

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
