use gouda_ecs::{ECS, GameSceneId};
use crate::input::{GameInput, LetterKeys};
use crate::platform::PlatformLayer;
use crate::window::{WindowProps, WindowEvent};
use std::time;
use std::time::Instant;
use gouda_rendering::{Scene, Renderer};
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[macro_use]
extern crate enum_map;

pub mod input;
mod platform;
pub mod window;
use gouda_rendering::camera::{Camera};
pub use gouda_images::bmp as bmp;
pub use gouda_images::png as png;
use crate::imgui::renderer::GoudaImguiRenderer;
use crate::imgui::platform::GoudaImguiPlatform;
use ::imgui::*;
pub mod gui;
pub mod mouse_capture;
pub mod imgui;

pub type RenderLayer = String;
pub type RenderOrder = u32;

pub struct QuitEvent;



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
                data: include_bytes!("../../../assets/Roboto-Regular.ttf"),
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

        let imgui_renderer = GoudaImguiRenderer::create(&renderer, &mut imgui);

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
                    WindowEvent::ResizeEvent { width: _, height: _ } => {
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
