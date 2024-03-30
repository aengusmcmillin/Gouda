
use std::{
    collections::HashMap, default, env, rc::Rc
};

use cgmath::{Matrix4, SquareMatrix};
use gouda::{
    camera::{Camera, PerspectiveCamera},
    ecs::{Entity, GameSceneId, Mutation, Mutations, ECS},
    rendering::{obj::{load_mtl_file, load_obj_file, ObjMesh}, Renderer, Scene},
    transform::TransformComponent,
    window::WindowProps,
    GameLogic, GameScene, Gouda, RenderLayer,
};


pub const START_MENU_SCENE: GameSceneId = 0;

pub struct MainGameScene {}

#[derive(Debug)]
struct GreenLevel {
    green_value: f32,
    time_value: f32,
}

struct TickMutation {
    entity: Entity,
    new_time: f32,
}

impl Mutation for TickMutation {
    fn apply(&self, ecs: &mut ECS) {
        let green = ecs.write::<GreenLevel>(&self.entity).unwrap();
        green.time_value = self.new_time;
        green.green_value = f32::sin(green.time_value) / 2. + 0.5;
    }
}

pub fn tick_system(ecs: &ECS, dt: f32) -> Mutations {
    let mut mutations: Mutations = Vec::new();
    for (green, ent) in ecs.read1::<GreenLevel>() {
        mutations.push(Box::new(TickMutation {
            entity: ent,
            new_time: green.time_value + dt,
        }));
    }
    mutations
}

impl GameScene for MainGameScene {
    fn on_scene_start(&self, ecs: &mut ECS) {
        ecs.build_entity()
            .add(Camera::Perspective(PerspectiveCamera::new(1.)))
            .add(TransformComponent::builder().position3d(0., 0., 15.).build());

        ecs.build_entity().add(GreenLevel {green_value: 0., time_value: 0.});

        ecs.add_system(Box::new(tick_system));

        let renderer = ecs.read_res::<Rc<Renderer>>();
        let objfile = load_obj_file("./assets/models/tree.obj").unwrap();
        let mtlfile = load_mtl_file("./assets/models/tree.mtl").unwrap();
        let model = ObjMesh::new(renderer, objfile, mtlfile);
        ecs.add_res(model);
    }

    fn on_scene_stop(&self, _ecs: &mut ECS) {}

    fn render_scene(&self, ecs: &ECS, scene: &Scene) {

        for (green, ent) in ecs.read1::<GreenLevel>() {
            let transform1 = TransformComponent::builder()
                .position(-0.2, 0.)
                .scale(0.1, 0.8)
                .rotation(0.3, 0.)
                .build();
            let transform2 = TransformComponent::builder()
                .position3d(0., 0., -1.)
                .scale(0.3, 0.3)
                .rotation3d(30., 30., 30.)
                .build();
            let transform3 = TransformComponent::builder()
                .position(0., 0.35)
                .scale(0.4, 0.1)
                .rotation(0.3, 0.3)
                .build();

            scene.submit_shape_by_name("quad", "cube", transform2.transform_matrix(), [0.5, 0.5, 0.5, 1.]);

        }
        let mesh = ecs.read_res::<ObjMesh>();
            let transform3 = TransformComponent::builder()
                .position(0., 0.)
                .scale(0.2, 0.2)
                .rotation(0., 0.0)
                .build();
        scene.submit_obj(mesh, transform3.transform_matrix());
    }

    fn next_scene(&self, _ecs: &ECS) -> Option<u32> {
        None
    }

    fn active_layers(&self, _ecs: &ECS) -> Vec<RenderLayer> {
        vec![]
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
            title: "Learn OpenGL in Gouda".to_string(),
            ..Default::default()
        }
    }

    fn register_events(&self, _ecs: &mut ECS) {}

    fn migrate_events(&self, _ecs: &mut ECS) {}

    fn game_scenes(&self) -> HashMap<GameSceneId, Box<dyn GameScene>> {
        let mut res: HashMap<GameSceneId, Box<dyn GameScene>> = HashMap::new();
        res.insert(START_MENU_SCENE, Box::new(MainGameScene {}));
        res
    }

    fn initial_game_scene(&self) -> u32 {
        START_MENU_SCENE
    }

    fn setup(&mut self, _ecs: &mut ECS) {}
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let mut gouda = Gouda::new(Game::new());
    gouda.run();
}
