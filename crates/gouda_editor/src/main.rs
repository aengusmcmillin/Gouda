use std::{collections::HashMap, env};

use editorlayer::EditorLayer;
use gouda::{camera::{Camera, OrthographicCamera}, ecs::{GameSceneId, ECS}, rendering::Scene, transform::TransformComponent, window::WindowProps, GameLogic, GameScene, Gouda, QuitEvent, RenderLayer};

pub mod editorlayer;

pub const START_MENU_SCENE: GameSceneId = 0;

pub struct StartMenuScene {}

impl GameScene for StartMenuScene {
    fn on_scene_start(&self, ecs: &mut ECS) {
        ecs.build_entity()
            .add(OrthographicCamera::new(8.))
            .add(TransformComponent::builder().build());
    }

    fn on_scene_stop(&self, ecs: &mut ECS) {
    }

    fn render_scene(&self, ecs: &ECS, scene: &Scene) {
    }

    fn next_scene(&self, ecs: &ECS) -> Option<u32> {
        return None;
    }

    fn active_layers(&self, _ecs: &ECS) -> Vec<RenderLayer> {
        return vec![String::from("Editor")];
    }

    fn camera(&self, ecs: &ECS) -> Box<dyn Camera> {
        let cam = ecs.read1::<OrthographicCamera>()[0].0.clone();
        return Box::new(cam);
    }
}
struct Game {}

impl Game {
    pub fn new() -> Self {
        Game {}
    }
}

impl GameLogic for Game {
    fn window_props(&self) -> WindowProps {
        WindowProps {
            width: 900.0,
            height: 900.0,
            title: "Gouda Editor".to_string(),
            target_ms_per_frame: 30.0,
        }
    }

    fn register_events(&self, ecs: &mut ECS) {
        ecs.register_event_type::<QuitEvent>();
    }

    fn migrate_events(&self, ecs: &mut ECS) {
        ecs.migrate_events::<QuitEvent>();
    }

    fn game_scenes(&self) -> HashMap<GameSceneId, Box<dyn GameScene>> {
        let mut res: HashMap<GameSceneId, Box<dyn GameScene>> = HashMap::new();
        res.insert(START_MENU_SCENE, Box::new(StartMenuScene {}));
        return res;
    }

    fn initial_game_scene(&self) -> u32 {
        return START_MENU_SCENE;
    }

    fn setup(&mut self, ecs: &mut ECS) {
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut gouda = Gouda::new(Game::new());
    gouda.push_layer(Box::new(EditorLayer::new()));
    gouda.run();
}
