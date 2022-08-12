use crate::ecs::{ECS, GameStateId};
use crate::input::{GameInput, LetterKeys};
use crate::platform::PlatformLayer;
use crate::window::{WindowProps, WindowEvent};
use std::thread::sleep;
use std::time;
use std::time::Instant;
use crate::rendering::{Scene, Renderer};
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[macro_use]
extern crate enum_map;

pub mod genindex;
pub mod ecs;
pub mod input;
mod platform;
pub mod window;
pub mod rendering;
pub mod math;
mod utils;
pub mod images;
use camera::{CameraT};
pub use images::bmp as bmp;
pub use images::png as png;
pub mod font;
pub mod types;
pub mod gui;
pub mod mouse_capture;
pub mod camera;
pub mod shader_lib;

pub type RenderLayer = String;
pub type RenderOrder = u32;

pub struct QuitEvent;

#[derive(Debug, Clone, Copy)]
pub struct TransformComponent {
    pub x: f32,
    pub y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rot_x: f32,
    pub rot_y: f32,
}

impl TransformComponent {
    pub fn change_pos(&mut self, dx: f32, dy: f32) {
        //let deg = (dy / dx).atan() / (std::f32::consts::PI / 180.);
        self.x += dx;
        self.y += dy;
    }

    pub fn builder() -> TransformComponentBuilder {
        TransformComponentBuilder::new()
    }
}

#[derive(Default)]
pub struct TransformComponentBuilder {
    pub x: f32,
    pub y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rot_x: f32,
    pub rot_y: f32,
}

impl TransformComponentBuilder {
    pub fn new() -> TransformComponentBuilder {
        TransformComponentBuilder {
            x: 0.,
            y: 0.,
            scale_x: 1.,
            scale_y: 1.,
            rot_x: 0.,
            rot_y: 0.,
        }
    }

    pub fn location(mut self, x: f32, y: f32) -> TransformComponentBuilder {
        self.x = x;
        self.y = y;
        self
    }

    pub fn scale(mut self, scale_x: f32, scale_y: f32) -> TransformComponentBuilder {
        self.scale_x = scale_x;
        self.scale_y = scale_y;
        self
    }

    pub fn rotation(mut self, rot_x: f32, rot_y: f32) -> TransformComponentBuilder {
        self.rot_x = rot_x;
        self.rot_y = rot_y;
        self
    }

    pub fn build(self) -> TransformComponent {
        TransformComponent { x: self.x, y: self.y, scale_x: self.scale_x, scale_y: self.scale_y, rot_x: self.rot_x, rot_y: self.rot_y }
    }
}

pub trait GameState {
    fn on_state_start(&self, ecs: &mut ECS);
    fn on_state_stop(&self, ecs: &mut ECS);
    fn render_state(&self, ecs: &ECS, scene: &Scene);
    fn next_state(&self, ecs: &ECS) -> Option<GameStateId>;
    fn active_layers(&self, ecs: &ECS) -> Vec<RenderLayer>;
    fn camera(&self, ecs: &ECS) -> Box<dyn CameraT>;
}

pub trait GameLogic {
    fn window_props(&self) -> WindowProps;
    fn cleanup_components(&self, ecs: &mut ECS);
    fn register_events(&self, ecs: &mut ECS);
    fn migrate_events(&self, ecs: &mut ECS);
    fn game_states(&self) -> HashMap<GameStateId, Box<dyn GameState>>;
    fn initial_game_state(&self) -> GameStateId;
    fn setup(&mut self, ecs: &mut ECS);
}

pub struct Gouda<T: GameLogic> {
    game_logic: T,
    ecs: ECS,
    game_states: HashMap<GameStateId, Box<dyn GameState>>,
    active_state: Option<GameStateId>,
}

impl<T: GameLogic> Gouda<T> {
    pub fn new(game_logic: T) -> Self {
        Gouda {
            game_logic,
            ecs: ECS::new(),
            game_states: HashMap::new(),
            active_state: None,
        }
    }

    fn setup_engine(&mut self) {
        self.ecs.add_res(GameInput::new());
    }

    fn setup_game(&mut self) {
        self.game_logic.register_events(&mut self.ecs);
        self.game_logic.setup(&mut self.ecs);

        self.game_states.extend(self.game_logic.game_states());
        let state = self.game_logic.initial_game_state();
        self.active_state = Some(state);
        self.game_states.get(&state).unwrap().on_state_start(&mut self.ecs);
    }

    fn get_active_state(&self) -> &Box<dyn GameState> {
        self.game_states.get(&self.active_state.unwrap()).unwrap()
    }

    fn update(&mut self, game_input: GameInput, events: Vec<WindowEvent>) {
        if let Some(state) = self.get_active_state().next_state(&self.ecs) {
            let gstate = self.game_states.get(&self.active_state.unwrap());
            if let Some(gstate) = gstate {
                gstate.on_state_stop(&mut self.ecs);
                self.ecs.clear_systems();
            }
            self.active_state = Some(state);
            let gstate = self.game_states.get(&self.active_state.unwrap());
            if let Some(gstate) = gstate {
                gstate.on_state_start(&mut self.ecs);
            }
        }

        if game_input.keyboard.cmd_down && game_input.keyboard.letter_down(LetterKeys::Q) {
            self.ecs.push_event(QuitEvent);
        }

        (*self.ecs.write_res::<GameInput>()) = game_input;
        (*self.ecs.write_res::<Vec<WindowEvent>>()) = events;

        self.ecs.run_systems();
        self.game_logic.cleanup_components(&mut self.ecs);
    }

    pub fn run(&mut self) {
        self.setup_engine();

        let props = self.game_logic.window_props();

        let mut platform = PlatformLayer::new(props);

        let mut now = Instant::now();

        let renderer = platform.get_renderer();

        self.ecs.add_res(renderer.clone());

        self.setup_game();

        loop {
            self.game_logic.migrate_events(&mut self.ecs);
            let window = platform.get_window();
            let input = window.capture_input();
            let events = window.capture_events();
            for event in &events {
                match event {
                    WindowEvent::CloseEvent => {
                        return;
                    }
                    WindowEvent::ResizeEvent { width, height } => {
                        self.ecs.remove_res::<Rc<Renderer>>();
                        platform.get_mut_renderer().resize(*width, *height);
                        let renderer = platform.get_renderer();
                        self.ecs.add_res(renderer.clone());
                    }
                };
            }
            self.update(input.clone(), events);
            if self.ecs.events::<QuitEvent>().len() > 0 {
                return;
            }

            let game_state = self.game_states.get(&self.active_state.unwrap()).unwrap();
            let renderer = platform.get_renderer();
            let camera = game_state.camera(&self.ecs);
            if let Some(scene) = renderer.begin_scene(camera) {
                game_state.render_state(&self.ecs, &scene);
                scene.end();
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
