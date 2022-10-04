use crate::ecs::{ECS, GameSceneId};
use crate::input::{GameInput, LetterKeys};
use crate::platform::PlatformLayer;
use crate::window::{WindowProps, WindowEvent};
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
use camera::{Camera};
use cgmath::{Matrix4, Vector3, Deg, Vector2};
pub use images::bmp as bmp;
pub use images::png as png;
use crate::imgui::renderer::GoudaImguiRenderer;
use crate::imgui::platform::GoudaImguiPlatform;
use ::imgui::*;
pub mod font;
pub mod font_library;
pub mod types;
pub mod gui;
pub mod mouse_capture;
pub mod camera;
pub mod shader_lib;
pub mod imgui;

pub type RenderLayer = String;
pub type RenderOrder = u32;

pub struct QuitEvent;


#[derive(Debug, Clone, Copy)]
pub struct TransformComponent {
    pub position: Vector2<f32>,
    pub rotation: Vector2<f32>,
    pub scale: Vector2<f32>,
}

impl TransformComponent {
    pub fn change_pos(&mut self, dx: f32, dy: f32) {
        self.position = self.position + Vector2::new(dx, dy);
    }

    pub fn builder() -> TransformComponentBuilder {
        TransformComponentBuilder::new()
    }

    pub fn transform_matrix(&self) -> Matrix4<f32> {
        return Matrix4::from_translation(Vector3::new(self.position.x, self.position.y, 0.)) * 
            Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, 1.) *
            Matrix4::from_angle_x(Deg(self.rotation.x)) *
            Matrix4::from_angle_y(Deg(self.rotation.y))
    }
}

pub struct TransformComponentBuilder {
    pub position: Vector2<f32>,
    pub rotation: Vector2<f32>,
    pub scale: Vector2<f32>,
}

impl TransformComponentBuilder {
    pub fn new() -> TransformComponentBuilder {
        TransformComponentBuilder {
            position: Vector2::new(0., 0.),
            rotation: Vector2::new(0., 0.),
            scale: Vector2::new(1., 1.),
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> TransformComponentBuilder {
        self.position = Vector2::new(x, y);
        self
    }

    pub fn scale(mut self, scale_x: f32, scale_y: f32) -> TransformComponentBuilder {
        self.scale = Vector2::new(scale_x, scale_y);
        self
    }

    pub fn rotation(mut self, rot_x: f32, rot_y: f32) -> TransformComponentBuilder {
        self.rotation = Vector2::new(rot_x, rot_y);
        self
    }

    pub fn build(self) -> TransformComponent {
        TransformComponent {
            position: self.position,
            scale: self.scale,
            rotation: self.rotation
        }
    }
}

pub trait GameScene {
    fn on_scene_start(&self, ecs: &mut ECS);
    fn on_scene_stop(&self, ecs: &mut ECS);
    fn render_scene(&self, ecs: &ECS, scene: &Scene);
    fn next_scene(&self, ecs: &ECS) -> Option<GameSceneId>;
    fn active_layers(&self, ecs: &ECS) -> Vec<RenderLayer>;
    fn camera(&self, ecs: &ECS) -> Box<dyn Camera>;
}

pub trait GameLogic {
    fn window_props(&self) -> WindowProps;
    fn register_events(&self, ecs: &mut ECS);
    fn migrate_events(&self, ecs: &mut ECS);
    fn game_scenes(&self) -> HashMap<GameSceneId, Box<dyn GameScene>>;
    fn initial_game_scene(&self) -> GameSceneId;
    fn setup(&mut self, ecs: &mut ECS);
}

pub struct Gouda<T: GameLogic> {
    game_logic: T,
    ecs: ECS,
    game_scenes: HashMap<GameSceneId, Box<dyn GameScene>>,
    active_scene: Option<GameSceneId>,
}

impl<T: GameLogic> Gouda<T> {
    pub fn new(game_logic: T) -> Self {
        Gouda {
            game_logic,
            ecs: ECS::new(),
            game_scenes: HashMap::new(),
            active_scene: None,
        }
    }

    fn setup_engine(&mut self) {
        self.ecs.add_res(GameInput::new());
    }

    fn setup_game(&mut self) {
        self.game_logic.register_events(&mut self.ecs);
        self.game_logic.setup(&mut self.ecs);

        self.game_scenes.extend(self.game_logic.game_scenes());
        let scene = self.game_logic.initial_game_scene();
        self.active_scene = Some(scene);
        self.game_scenes.get(&scene).unwrap().on_scene_start(&mut self.ecs);
    }

    fn get_active_scene(&self) -> &Box<dyn GameScene> {
        self.game_scenes.get(&self.active_scene.unwrap()).unwrap()
    }

    fn update(&mut self, dt: f32, game_input: GameInput, events: Vec<WindowEvent>) {
        if let Some(scene) = self.get_active_scene().next_scene(&self.ecs) {
            let gscene = self.game_scenes.get(&self.active_scene.unwrap());
            if let Some(gscene) = gscene {
                gscene.on_scene_stop(&mut self.ecs);
                self.ecs.clear_systems();
            }
            self.active_scene = Some(scene);
            let gscene = self.game_scenes.get(&self.active_scene.unwrap());
            if let Some(gscene) = gscene {
                gscene.on_scene_start(&mut self.ecs);
            }
        }

        if game_input.keyboard.cmd_down && game_input.keyboard.letter_down(LetterKeys::Q) {
            self.ecs.push_event(QuitEvent);
        }

        (*self.ecs.write_res::<GameInput>()) = game_input;
        (*self.ecs.write_res::<Vec<WindowEvent>>()) = events;

        self.ecs.run_systems(dt);
    }

    pub fn run(&mut self) {
        self.setup_engine();

        let props = self.game_logic.window_props();

        let mut platform = PlatformLayer::new(props);

        let mut now = Instant::now();

        let renderer = platform.get_renderer();

        let mut imgui = Context::create();
        imgui.set_ini_filename(None);

        GoudaImguiPlatform::init(&mut imgui);

        imgui.fonts().add_font(&[
            FontSource::TtfData {
                data: include_bytes!("../res/Roboto-Regular.ttf"),
            size_pixels: 13.0,
            config: Some(FontConfig {
                // As imgui-glium-renderer isn't gamma-correct with
                // it's font rendering, we apply an arbitrary
                // multiplier to make the font a bit "heavier". With
                // default imgui-glow-renderer this is unnecessary.
                rasterizer_multiply: 1.5,
                // Oversampling font helps improve text rendering at
                // expense of larger font atlas texture.
                oversample_h: 4,
                oversample_v: 4,
                ..FontConfig::default()
            }),
            }
        ]);

        let mut imgui_renderer = GoudaImguiRenderer::create(&renderer, &mut imgui);


        self.ecs.add_res(renderer.clone());

        self.setup_game();

        loop {
            let next = Instant::now();
            let delta = next - now;
            let target_dur = time::Duration::from_millis(16);
            if target_dur > delta {
                continue
            }
            let dt = delta.as_millis() as f32 / 1000.;
            now = next;


            self.game_logic.migrate_events(&mut self.ecs);
            let window = platform.get_window();

            GoudaImguiPlatform::prepare_frame(&mut imgui, &window, target_dur);

            let input = window.capture_input();
            let events = window.capture_events();
            for event in &events {
                match event {
                    WindowEvent::CloseEvent => {
                        return;
                    }
                    WindowEvent::ResizeEvent { width, height } => {
                        self.ecs.remove_res::<Rc<Renderer>>();
                        // platform.get_mut_renderer().resize(*width, *height);
                        let renderer = platform.get_renderer();
                        self.ecs.add_res(renderer.clone());
                    }
                };
            }
            self.update(dt, input.clone(), events);
            if self.ecs.events::<QuitEvent>().len() > 0 {
                return;
            }

            {
                let io  = imgui.io_mut();
                io.mouse_pos = [input.mouse.x as f32, input.mouse.y as f32];
                io.mouse_down[0] = input.mouse.buttons[0].ended_down;
            }

            let ui = imgui.frame();

            let draw_data = ui.render();

            let game_scene = self.game_scenes.get(&self.active_scene.unwrap()).unwrap();
            let renderer = platform.get_renderer();
            let camera = game_scene.camera(&self.ecs);
            if let Some(scene) = renderer.begin_scene(camera) {
                game_scene.render_scene(&self.ecs, &scene);
                imgui_renderer.render(&scene, draw_data);
                scene.end();
            }
        }
    }
}
