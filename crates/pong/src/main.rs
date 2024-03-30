use core::time;
use std::{
    collections::{HashMap, VecDeque},
    env,
    time::SystemTime,
};

use gouda::{
    camera::{Camera, OrthographicCamera},
    ecs::{Entity, EntityBuilder, GameSceneId, Mutation, Mutations, ECS},
    input::{AnyKey, GameInput, LetterKeys, SpecialKeys},
    rendering::{sprites::{ColorBoxComponent, SpriteSheetComponent}, Scene},
    transform::{self, TransformComponent},
    window::WindowProps,
    GameLogic, GameScene, Gouda, QuitEvent, RenderLayer,
};

pub const MAIN_GAME_SCENE: GameSceneId = 0;
pub const START_MENU_SCENE: GameSceneId = 1;
pub const GAME_OVER_SCENE: GameSceneId = 2;

#[derive(Debug)]
struct Controls {
    up: AnyKey,
    down: AnyKey,
}

#[derive(Debug)]
struct Collider {
    width: f32,
    height: f32,
}

#[derive(Debug)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[derive(Debug)]
struct Paddle {}

impl Paddle {
    pub fn createp1(ecs: &mut ECS) {
        let color = ColorBoxComponent::new(ecs, [0.3, 0.3, 0.3]);
        let transform = TransformComponent::builder()
            .position(-7., 0.)
            .scale(0.3, 0.8)
            .build();
        let velocity = Velocity { dx: 0., dy: 0. };
        let collider = Collider {
            width: 0.3,
            height: 0.8,
        };
        let controls = Controls {
            up: AnyKey::Letter(LetterKeys::W),
            down: AnyKey::Letter(LetterKeys::S),
        };

        ecs.build_entity()
            .add(color)
            .add(collider)
            .add(velocity)
            .add(transform)
            .add(controls)
            .add(Paddle {});
    }

    pub fn createp2(ecs: &mut ECS) {
        let color = ColorBoxComponent::new(ecs, [0.3, 0.3, 0.3]);
        let transform = TransformComponent::builder()
            .position(7., 0.)
            .scale(0.3, 0.8)
            .build();
        let velocity = Velocity { dx: 0., dy: 0. };
        let collider = Collider {
            width: 0.3,
            height: 0.8,
        };
        let controls = Controls {
            up: AnyKey::Special(SpecialKeys::UpArrow),
            down: AnyKey::Special(SpecialKeys::DownArrow),
        };
        ecs.build_entity()
            .add(color)
            .add(collider)
            .add(velocity)
            .add(transform)
            .add(controls)
            .add(Paddle {});
    }

}

#[derive(Debug)]
struct Ball {}

impl Ball {
    pub fn create(ecs: &mut ECS) {
        let color = ColorBoxComponent::new(ecs, [0.3, 0.3, 0.3]);
        let transform = TransformComponent::builder()
            .position(0., 0.)
            .scale(0.3, 0.3)
            .build();
        let velocity = Velocity { dx: -3., dy: 3. };
        let collider = Collider {
            width: 0.3,
            height: 0.3,
        };
        ecs.build_entity()
            .add(color)
            .add(collider)
            .add(velocity)
            .add(transform)
            .add(Ball {});
    }
}


struct ApplyMovementMutation {
    entity: Entity,
    dy: f32,
    dx: f32,
}

impl Mutation for ApplyMovementMutation {
    fn apply(&self, ecs: &mut ECS) {
        let transform = ecs.write::<TransformComponent>(&self.entity).unwrap();
        transform.change_pos(self.dx, self.dy);
    }
}

pub fn apply_velocity_system(ecs: &ECS, dt: f32) -> Mutations {
    let mut mutations: Mutations = Vec::new();
    for (vel, ent) in ecs.read1::<Velocity>() {
        mutations.push(Box::new(ApplyMovementMutation {
            entity: ent,
            dy: vel.dy * dt,
            dx: vel.dx * dt,
        }));
    }
    return mutations;
}

fn collides(
    c1: &Collider,
    t1: &TransformComponent,
    c2: &Collider,
    t2: &TransformComponent,
) -> bool {
    let lc1 = t1.position[0] - c1.width / 2.;
    let rc1 = lc1 + c1.width;
    let lc2 = t2.position[0] - c2.width / 2.;
    let rc2 = lc2 + c2.width;

    let bc1 = t1.position[1] - c1.height / 2.;
    let tc1 = bc1 + c1.height;
    let bc2 = t2.position[1] - c2.height / 2.;
    let tc2 = bc2 + c2.height;

    return rc1 >= lc2 && lc1 <= rc2 && bc1 <= tc2 && tc1 >= bc2;
}


pub fn ball_collision_handling_system(ecs: &ECS, _dt: f32) -> Mutations {
    let mut mutations: Mutations = Vec::new();
    for (_, ball_velocity, ball_collider, ball_transform, ball_ent) in ecs.read4::<Ball, Velocity, Collider, TransformComponent>() {
        for (_, paddle_collider, paddle_transform, _) in ecs.read3::<Paddle, Collider, TransformComponent>() {
            if collides(ball_collider, ball_transform, paddle_collider, paddle_transform) {
                // if ball_transform.position.y < paddle_transform.position.y || ball_transform.position.y > paddle_transform.position.y + paddle_transform.scale.y {
                //     mutations.push(Box::new(ChangeBallVelocityMutation { entity: ball_ent, new_velocity: [ball_velocity.dx, -ball_velocity.dy]}));
                // } else {
                    mutations.push(Box::new(ChangeBallVelocityMutation { entity: ball_ent, new_velocity: [-ball_velocity.dx, ball_velocity.dy]}));
                // }
            }
        }

        if ball_transform.position[1] < (-8. + ball_transform.scale[1] * 2.) {
            mutations.push(Box::new(ChangeBallVelocityMutation { entity: ball_ent, new_velocity: [ball_velocity.dx, -ball_velocity.dy]}));
        } else if ball_transform.position[1] > (8. - ball_transform.scale[1] / 2.) {
            mutations.push(Box::new(ChangeBallVelocityMutation { entity: ball_ent, new_velocity: [ball_velocity.dx, -ball_velocity.dy]}));
        }
    }
    return mutations;
}

struct JumpMutation {
    entity: Entity,
}

impl Mutation for JumpMutation {
    fn apply(&self, ecs: &mut ECS) {
        let velocity = ecs.write::<Velocity>(&self.entity).unwrap();
        velocity.dy = 5.;
    }
}

pub fn jump_system(ecs: &ECS, _dt: f32) -> Mutations {
    let input = ecs.read_res::<GameInput>();
    let mut mutations: Mutations = Vec::new();
    for (_, controls, ent) in ecs.read2::<Paddle, Controls>() {
        if input.keyboard.key_down(controls.up) {
            mutations.push(Box::new(ApplyMovementMutation { entity: ent, dy: 0.1, dx: 0.}))
        }
        if input.keyboard.key_down(controls.down) {
            mutations.push(Box::new(ApplyMovementMutation { entity: ent, dy: -0.1, dx: 0.}))
        }
    }
    return mutations;
}

struct ChangeBallVelocityMutation {
    entity: Entity,
    new_velocity: [f32; 2]
}

impl Mutation for ChangeBallVelocityMutation {
    fn apply(&self, ecs: &mut ECS) {
        if let Some(v) = ecs.write::<Velocity>(&self.entity) {
            v.dx = self.new_velocity[0];
            v.dy = self.new_velocity[1];
        }
    }
}

pub struct GameOverScene {}

impl GameScene for GameOverScene {
    fn on_scene_start(&self, ecs: &mut ECS) {
        todo!()
    }

    fn on_scene_stop(&self, ecs: &mut ECS) {
        todo!()
    }

    fn render_scene(&self, ecs: &ECS, scene: &Scene) {
        todo!()
    }

    fn next_scene(&self, ecs: &ECS) -> Option<GameSceneId> {
        todo!()
    }

    fn active_layers(&self, ecs: &ECS) -> Vec<RenderLayer> {
        todo!()
    }

    fn camera(&self, ecs: &ECS) -> Box<dyn Camera> {
        todo!()
    }
}
pub struct MainGameScene {}

impl GameScene for MainGameScene {
    fn on_scene_start(&self, ecs: &mut ECS) {
        ecs.build_entity()
            .add(OrthographicCamera::new(8.))
            .add(TransformComponent::builder().build());

        Paddle::createp1(ecs);
        Paddle::createp2(ecs);
        Ball::create(ecs);

        ecs.add_system(Box::new(apply_velocity_system));
        ecs.add_system(Box::new(jump_system));
        ecs.add_system(Box::new(ball_collision_handling_system));
        // Create Ground
        // Setup Pipe Creation System
        // Setup Movement System
        //
    }

    fn on_scene_stop(&self, ecs: &mut ECS) {}

    fn render_scene(&self, ecs: &ECS, scene: &Scene) {
        // Draw background
        // Draw ground
        // Draw bird
        // Draw pipes
        for (location, color_box, _) in ecs.read2::<TransformComponent, ColorBoxComponent>() {
            color_box.draw(&scene, location);
        }

        for (location, spritesheet, _) in ecs.read2::<TransformComponent, SpriteSheetComponent>() {
            spritesheet.draw(&scene, location);
        }
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
            title: "Flappy Cheese".to_string(),
            target_ms_per_frame: 30.0,
        }
    }

    fn register_events(&self, ecs: &mut ECS) {}

    fn migrate_events(&self, ecs: &mut ECS) {}

    fn game_scenes(&self) -> HashMap<GameSceneId, Box<dyn GameScene>> {
        let mut res: HashMap<GameSceneId, Box<dyn GameScene>> = HashMap::new();
        res.insert(START_MENU_SCENE, Box::new(MainGameScene {}));
        return res;
    }

    fn initial_game_scene(&self) -> u32 {
        return START_MENU_SCENE;
    }

    fn setup(&mut self, ecs: &mut ECS) {}
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut gouda = Gouda::new(Game::new());
    gouda.run();
}
