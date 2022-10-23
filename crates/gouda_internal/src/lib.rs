use cgmath::Transform;
use gouda_ecs::{GameSceneId, ECS};
use gouda_editor::EditorLayer;
use gouda_input::{GameInput, LetterKeys};
use gouda_layer::Layer;
use gouda_platform::PlatformLayer;
use gouda_rendering::font_library::FontLibrary;
use gouda_rendering::shader_lib::ShaderLibrary;
use gouda_rendering::shapes::ShapeLibrary;
use gouda_rendering::{Renderer, Scene};
use gouda_transform::TransformComponent;
use gouda_window::{WindowEvent, WindowProps};
use std::collections::HashMap;
use std::rc::Rc;
use std::time;
use std::time::Instant;

pub use gouda_images::{bmp, png};
use gouda_rendering::camera::{Camera, OrthographicCamera};
pub mod gui;
pub mod mouse_capture;

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
    layers: Vec<Box<dyn Layer>>,
    pub shader_lib: Option<ShaderLibrary>,
    pub shape_lib: Option<ShapeLibrary>,
    pub font_lib: Option<FontLibrary>,
}

impl<T: GameLogic> Gouda<T> {
    pub fn new(game_logic: T) -> Self {
        Gouda {
            game_logic,
            ecs: ECS::new(),
            game_scenes: HashMap::new(),
            active_scene: None,
            layers: vec![Box::new(EditorLayer::new())],
            shader_lib: None,
            shape_lib: None,
            font_lib: None,
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
        self.game_scenes
            .get(&scene)
            .unwrap()
            .on_scene_start(&mut self.ecs);
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
        self.ecs.add_res(renderer.clone());

        let ecs = &self.ecs;
        self.layers.iter_mut().for_each(|layer| layer.setup(ecs));

        self.setup_game();

        loop {
            let next = Instant::now();
            let delta = next - now;
            let target_dur = time::Duration::from_millis(16);
            if target_dur > delta {
                continue;
            }
            let dt = delta.as_millis() as f32 / 1000.;
            now = next;

            self.game_logic.migrate_events(&mut self.ecs);
            let window = platform.get_window();

            let input = window.capture_input();
            let events = window.capture_events();
            for event in &events {
                match event {
                    WindowEvent::CloseEvent => {
                        return;
                    }
                    WindowEvent::ResizeEvent {
                        width: _,
                        height: _,
                    } => {
                        self.ecs.remove_res::<Rc<Renderer>>();
                        // platform.get_mut_renderer().resize(*width, *height);
                        let renderer = platform.get_renderer();
                        self.ecs.add_res(renderer.clone());
                    }
                };
            }

            let ecs = &self.ecs;
            self.layers
                .iter_mut()
                .for_each(|layer| layer.update(ecs, dt));

            self.update(dt, input.clone(), events);
            if self.ecs.events::<QuitEvent>().len() > 0 {
                return;
            }

            let game_scene = self.game_scenes.get(&self.active_scene.unwrap()).unwrap();
            let renderer = platform.get_renderer();
            let cameras = self.ecs.read2::<OrthographicCamera, TransformComponent>();
            let camera = cameras[0].0;
            let transform = cameras[0].1;
            if let Some(mut scene) = renderer.begin_scene() {
                scene.bind_camera(camera, transform);
                game_scene.render_scene(&self.ecs, &scene);
                let mut ecs = &mut self.ecs;
                self.layers
                    .iter_mut()
                    .for_each(|layer| layer.render(&mut ecs, &mut scene));
                scene.end();
            }
        }
    }
}
